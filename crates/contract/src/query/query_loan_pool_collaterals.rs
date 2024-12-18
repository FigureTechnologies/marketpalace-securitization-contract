use crate::core::aliases::ProvQueryResponse;
use crate::core::msg::QueryLoanPoolCollateralResponse;
use crate::storage::loan_pool_collateral::get_all_states;
use cosmwasm_std::{to_binary, Storage};

/// This function handles the process of getting all states from storage and
/// creates a `QueryLoanPoolCollateralResponse` with the resulting collaterals.
/// The response is then serialized into binary form.
///
/// # Arguments
///
/// * `storage` - A dynamic reference to the storage from which to get all states
///
/// # Returns
///
/// This function returns a `ProvQueryResponse` which is basically a Result type containing serialized
/// response in binary format or an error.
///
/// On successful operation, the function returns `Ok`, wrapping the binary form of the `QueryLoanPoolCollateralResponse`.
///
/// If there are any errors during the process (e.g., failure in getting states from storage, serializing the response),
/// it returns an `Err` wrapping the error.
///
/// # Example
///

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    Ok(to_binary(&QueryLoanPoolCollateralResponse {
        collaterals: get_all_states(storage),
    })?)
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
    use cosmwasm_std::{
        from_binary, testing::mock_env, to_json_binary, Addr, Binary, ContractResult, SystemResult,
    };
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::shim::Any;
    use provwasm_std::types::cosmos::base::v1beta1::Coin;
    use provwasm_std::types::provenance::marker::v1::{
        Balance, QueryHoldingRequest, QueryHoldingResponse, QueryMarkerRequest, QueryMarkerResponse,
    };

    #[test]
    fn test_all_collateral_success() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();

        let cb = Box::new(|bin: &Binary| -> SystemResult<ContractResult<Binary>> {
            let message = QueryMarkerRequest::try_from(bin.clone()).unwrap();
            let inner_deps = mock_provenance_dependencies();
            let expected_marker = MockMarker::new_owned_marker("contributor").to_marker_account();

            let response = QueryMarkerResponse {
                marker: Some(Any {
                    type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
                    value: expected_marker.to_proto_bytes(),
                }),
            };

            let binary = to_json_binary(&response).unwrap();
            SystemResult::Ok(ContractResult::Ok(binary))
        });

        deps.querier
            .registered_custom_queries
            .insert("/provenance.marker.v1.Query/Marker".to_string(), cb);

        let cb_holding = Box::new(|bin: &Binary| -> SystemResult<ContractResult<Binary>> {
            let message = QueryHoldingRequest::try_from(bin.clone()).unwrap();

            let response = if message.id == "markerdenom" {
                QueryHoldingResponse {
                    balances: vec![Balance {
                        address: Addr::unchecked("markerdenom").to_string(),
                        coins: vec![Coin {
                            denom: "markerdenom".to_string(),
                            amount: "1".to_string(),
                        }],
                    }],
                    pagination: None,
                }
            } else {
                panic!("unexpected query for denom")
            };

            let binary = to_json_binary(&response).unwrap();
            SystemResult::Ok(ContractResult::Ok(binary))
        });

        deps.querier.registered_custom_queries.insert(
            "/provenance.marker.v1.Query/Holding".to_string(),
            cb_holding,
        );

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
        if let Err(err) = &loan_pool_result {
            panic!("Error: {:?}", err);
        }

        // Assert that the result is not an error
        assert!(loan_pool_result.is_ok());

        //query all states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryCollaterals {}).unwrap();
        let value: QueryLoanPoolCollateralResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.collaterals.len());
    }

    #[test]
    fn test_all_collateral_none_exists() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");

        let cb = Box::new(|bin: &Binary| -> SystemResult<ContractResult<Binary>> {
            let message = QueryMarkerRequest::try_from(bin.clone()).unwrap();
            let inner_deps = mock_provenance_dependencies();
            let expected_marker = MockMarker::new_owned_marker("contributor").to_marker_account();

            let response = QueryMarkerResponse {
                marker: Some(Any {
                    type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
                    value: expected_marker.to_proto_bytes(),
                }),
            };

            let binary = to_json_binary(&response).unwrap();
            SystemResult::Ok(ContractResult::Ok(binary))
        });

        deps.querier
            .registered_custom_queries
            .insert("/provenance.marker.v1.Query/Marker".to_string(), cb);

        let cb_holding = Box::new(|bin: &Binary| -> SystemResult<ContractResult<Binary>> {
            let message = QueryHoldingRequest::try_from(bin.clone()).unwrap();

            let response = if message.id == "markerdenom" {
                QueryHoldingResponse {
                    balances: vec![Balance {
                        address: Addr::unchecked("markerdenom").to_string(),
                        coins: vec![Coin {
                            denom: "markerdenom".to_string(),
                            amount: "1".to_string(),
                        }],
                    }],
                    pagination: None,
                }
            } else {
                panic!("unexpected query for denom")
            };

            let binary = to_json_binary(&response).unwrap();
            SystemResult::Ok(ContractResult::Ok(binary))
        });

        deps.querier.registered_custom_queries.insert(
            "/provenance.marker.v1.Query/Holding".to_string(),
            cb_holding,
        );

        //query all states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryCollaterals {}).unwrap();
        let value: QueryLoanPoolCollateralResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.collaterals.len());
    }
}
