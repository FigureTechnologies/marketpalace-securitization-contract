use crate::core::aliases::ProvQueryResponse;
use crate::core::msg::QueryLoanPoolCollateralResponse;
use crate::storage::loan_pool_collateral::get_all_states;
use cosmwasm_std::{to_binary, Storage};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let loan_pool_collaterals = get_all_states(storage);
    let response = QueryLoanPoolCollateralResponse {
        collaterals: loan_pool_collaterals,
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use crate::contract::query;
    use crate::core::msg::{QueryLoanPoolCollateralResponse, QueryMsg};
    use crate::core::security::ContributeLoanPools;
    use crate::execute::settlement::add_loan_pool::handle as add_loanpool_handle;
    use crate::execute::settlement::whitelist_loanpool_contributors::handle as whitelist_loanpool_handle;
    use crate::util::mock_marker::MockMarker;
    use crate::util::testing::instantiate_contract;
    use cosmwasm_std::testing::mock_info;
    use cosmwasm_std::{from_binary, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_all_collateral() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();
        deps.querier.with_markers(vec![marker.clone()]);
        let env = mock_env();
        let info = mock_info("contributor", &[]);
        let info_white_list = mock_info("gp", &[]);
        let addr_contributor = Addr::unchecked("contributor");
        let white_list_addr = vec![addr_contributor.clone()];
        let whitelist_result =
            whitelist_loanpool_handle(deps.as_mut(), info_white_list.sender, white_list_addr);
        assert!(whitelist_result.is_ok());
        // Create a loan pool
        let loan_pools = ContributeLoanPools {
            markers: vec![marker_denom.clone()],
        };
        // Call the handle function
        let loan_pool_result =
            add_loanpool_handle(deps.as_mut(), env.to_owned(), info.clone(), loan_pools);
        // Assert that the result is not an error
        assert!(loan_pool_result.is_ok());

        //query all states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryCollaterals {}).unwrap();
        let value: QueryLoanPoolCollateralResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.collaterals.len());
    }

    #[test]
    fn test_all_collateral_zero_exists() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        deps.querier.with_markers(vec![marker.clone()]);

        //query all states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryCollaterals {}).unwrap();
        let value: QueryLoanPoolCollateralResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.collaterals.len());
    }
}
