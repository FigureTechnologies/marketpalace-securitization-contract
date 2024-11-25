use cosmwasm_std::{to_json_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryContractsResponse},
    storage,
};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let response = QueryContractsResponse {
        contracts: storage::contract::list(storage),
    };
    Ok(to_json_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::QueryContractsResponse,
        query::query_contracts::handle,
        util::testing::{add_contracts, create_admin_deps, instantiate_contract},
    };

    #[test]
    fn test_query_contracts_handles_empty() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let _ = instantiate_contract(deps.as_mut(), env).unwrap();
        let bin_response = handle(&deps.storage).unwrap();
        let response: QueryContractsResponse = from_json(&bin_response).unwrap();
        assert_eq!(0, response.contracts.len());
    }

    #[test]
    fn test_query_contracts() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let _ = instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts(deps.as_mut(), env).unwrap();
        let bin_response = handle(&deps.storage).unwrap();
        let response: QueryContractsResponse = from_json(&bin_response).unwrap();
        assert_eq!(
            vec![
                Addr::unchecked("contract1"),
                Addr::unchecked("contract2"),
                Addr::unchecked("contract3")
            ],
            response.contracts
        );
    }
}
