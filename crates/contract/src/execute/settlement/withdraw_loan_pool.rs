use crate::core::collateral::{AccessGrantSerializable, LoanPoolMarkers, LoanPoolRemovalData};
use crate::core::security::WithdrawLoanPools;
use crate::storage::loan_pool_collateral::{get, remove};
use crate::util::provenance_utilities::{
    get_marker, get_marker_address, release_marker_from_contract,
};
use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::state::{self},
};
use cosmwasm_std::{to_json_binary, Addr, DepsMut, Env, Event, MessageInfo, Response};
use provwasm_std::types::provenance::marker::v1::{AccessGrant, MarkerQuerier};
use provwasm_std::types::provenance::metadata::v1::p8e::PartyType::Marker;

/// Handle function that processes a list of loan pools to be withdrawn.
///
/// This function is invoked when a loan pool is to be withdrawn. It performs operations
/// such as checking prerequisites i.e the sender must be the same as the initialized "gp",
/// releasing the marker to the contract, updating state accordingly by removing the marker
/// and updating the response.
///
/// Parameters:
/// * `deps`: the storage dependency object that gives access to the relevant dependencies.
/// * `env`: the environment details of the contract.
/// * `info`: the message information.
/// * `loan_pools`: withdraw loan pools object.
///
/// Returns:
/// * `ProvTxResponse`: the status of the operation in form of `ProvTxResponse` object.
///
/// # Panics
/// * if the sender is not the same as the "gp".
/// * if the marker address from the marker denom does not exist.
/// * if getting a marker by denom fails.
/// * if `deps.storage` is not populated.
pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    loan_pools: WithdrawLoanPools,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;

    // the gp can only release the pool
    if info.sender != state.gp {
        return Err(ContractError::Unauthorized {});
    }

    // create empty response object
    let mut response = Response::new();

    let mut collaterals = Vec::new();

    // Fetch removal data
    let removal_data: Vec<_> = loan_pools
        .markers
        .iter()
        .map(|pool| {
            let removal_data = withdraw_marker_pool_collateral(&deps, &env, pool.clone())?;
            Ok((
                removal_data.collateral.marker_address.to_owned(),
                removal_data,
            ))
        })
        .collect::<Result<_, ContractError>>()?;

    // Modify state
    for (
        address,
        LoanPoolRemovalData {
            collateral,
            messages,
        },
    ) in removal_data
    {
        remove(deps.storage, &collateral)?;

        // store each collateral in collaterals vector
        collaterals.push(collateral);

        response = response.add_messages(messages).add_event(
            Event::new("loan_pool_withdrawn").add_attribute("marker_address", address.to_string()),
        );
    }

    // Add removed_by attribute only if loan_pool_withdrawn event is added
    if response
        .events
        .iter()
        .any(|event| event.ty == "loan_pool_withdrawn")
    {
        response = response.add_attribute("action", "loan_pool_removed");
        response = response.add_attribute("loan_pool_removed_by", info.sender);
    }
    // Set response data to collaterals vector
    response = response.set_data(to_json_binary(&LoanPoolMarkers::new(collaterals))?);

    Ok(response)
}

fn withdraw_marker_pool_collateral(
    deps: &DepsMut,
    env: &Env,
    marker_denom: String,
) -> Result<LoanPoolRemovalData, ContractError> {
    // get marker
    let querier = MarkerQuerier::new(&deps.querier);
    let marker = get_marker(marker_denom, &querier)?;
    let marker_address = get_marker_address(marker.base_account)?;
    let collateral = get(deps.storage, Addr::unchecked(marker_address))?;
    let messages = release_marker_from_contract(
        marker.denom,
        &env.contract.address,
        &collateral
            .clone()
            .removed_permissions
            .into_iter()
            .map(AccessGrant::from)
            .collect::<Vec<_>>(),
    )?;
    Ok(LoanPoolRemovalData {
        collateral,
        messages,
    })
}

#[cfg(test)]
mod tests {
    use crate::core::collateral::{
        AccessGrantSerializable, LoanPoolMarkerCollateral, LoanPoolMarkers,
    };
    use crate::core::error::ContractError;
    use crate::core::security::{ContributeLoanPools, WithdrawLoanPools};
    use crate::execute::settlement::add_loan_pool::handle as add_loanpool_handle;
    use crate::execute::settlement::whitelist_loanpool_contributors::handle as whitelist_loanpool_handle;
    use crate::execute::settlement::withdraw_loan_pool::handle;
    use crate::util::mock_marker::MockMarker;
    use crate::util::testing::instantiate_contract;
    use cosmwasm_std::testing::{message_info, mock_env, mock_info};
    use cosmwasm_std::ReplyOn::Never;
    use cosmwasm_std::{
        from_json, to_json_binary, Addr, AnyMsg, Binary, ContractResult, SubMsg, SystemResult,
        Uint128,
    };
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::shim::Any;
    use provwasm_std::types::cosmos::auth::v1beta1::BaseAccount;
    use provwasm_std::types::cosmos::base::v1beta1::Coin;
    use provwasm_std::types::provenance::marker::v1::Access::{
        Admin, Burn, Delete, Deposit, Mint, Withdraw,
    };
    use provwasm_std::types::provenance::marker::v1::{
        AccessGrant, Balance, MarkerAccount, MarkerStatus, MarkerType, QueryHoldingRequest,
        QueryHoldingResponse, QueryMarkerRequest, QueryMarkerResponse,
    };

    #[test]
    fn test_handle_should_fail_when_sender_is_not_gp() {
        let mut deps = mock_provenance_dependencies();
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let env = mock_env();
        let info = message_info(&Addr::unchecked("someone"), &[]);
        // Create a loan pool
        let loan_pools = WithdrawLoanPools {
            markers: vec![marker_denom],
        };

        // Call the handle function
        let result = handle(deps.as_mut(), env, info, loan_pools);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContractError::Unauthorized {}
        ));
    }

    #[test]
    fn test_withdraw_loan_pool() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        let denom = marker.denom.to_owned();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("contributor"), &[]);
        let info_white_list = message_info(&Addr::unchecked("gp"), &[]);
        let addr_contributor = Addr::unchecked("contributor");
        let white_list_addr = vec![addr_contributor.clone()];
        let whitelist_result =
            whitelist_loanpool_handle(deps.as_mut(), info_white_list.sender, white_list_addr);
        assert!(whitelist_result.is_ok());
        match whitelist_result {
            Ok(response) => {
                for attribute in response.attributes.iter() {
                    if attribute.key == "action" {
                        assert_eq!(attribute.value, "whitelist_added");
                    } else if attribute.key == "address_whitelisted" {
                        // Verify if the addresses are correct
                        let whitelisted_addresses: Vec<&str> = attribute.value.split(",").collect();
                        assert_eq!(whitelisted_addresses, vec!["contributor"]);
                    }
                }
            }
            Err(e) => panic!("Error: {:?}", e),
        }
        // Create a loan pool
        let loan_pools = ContributeLoanPools {
            markers: vec![denom.to_owned()],
        };

        let expected_collaterals = vec![LoanPoolMarkerCollateral {
            marker_address: marker.address.clone(),
            marker_denom: denom.clone(),
            share_count: Uint128::new(100),
            original_contributor: info.sender.to_owned(),
            removed_permissions: if let Some(first_permission) = marker.permissions.first() {
                vec![AccessGrantSerializable::from(first_permission.clone())]
            } else {
                vec![]
            },
        }];

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
                            amount: "100".to_string(),
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

        // Call the handle function
        let loan_pool_result =
            add_loanpool_handle(deps.as_mut(), env.to_owned(), info.clone(), loan_pools);
        // Assert that the result is not an error
        assert!(loan_pool_result.is_ok());
        match loan_pool_result {
            Ok(response) => {
                // Checking response data
                let loan_pool_markers: LoanPoolMarkers =
                    from_json(&response.data.unwrap()).unwrap();
                assert_eq!(loan_pool_markers.collaterals, expected_collaterals); //replace `collaterals` with expected vec of collaterals

                // Checking response attributes and events
                let mut found_event = false;
                let mut found_attribute = false;

                for event in response.events.iter() {
                    if event.ty == "loan_pool_added" {
                        found_event = true;
                        // Check event attributes here if needed
                    }
                }

                for attribute in response.attributes.iter() {
                    if attribute.key == "loan_pool_added_by" {
                        assert_eq!(attribute.value, info.sender.clone().to_string());
                        found_attribute = true;
                    }
                }

                assert!(found_event, "Failed to find loan_pool_added event");
                assert!(found_attribute, "Failed to find added_by attribute");
            }
            Err(e) => panic!("Error: {:?}", e),
        }

        // Create a loan pool
        let withdraw_loan_pools = WithdrawLoanPools {
            markers: vec![denom.to_owned()],
        };
        let info = message_info(&Addr::unchecked("gp"), &[]);

        let withdraw_loan_pool_result = handle(
            deps.as_mut(),
            env.to_owned(),
            info.clone(),
            withdraw_loan_pools,
        );
        // Assert that the result is not an error
        assert!(withdraw_loan_pool_result.is_ok());
        match withdraw_loan_pool_result {
            Ok(response) => {
                // Checking response data
                let withdraw_loan_pool_result: LoanPoolMarkers =
                    from_json(&response.data.unwrap()).unwrap();
                assert_eq!(withdraw_loan_pool_result.collaterals, expected_collaterals); //replace `collaterals` with expected vec of collaterals
                assert_eq!(response.events.len(), 1);
                assert_eq!(response.attributes.len(), 2);
                // Checking response attributes and events
                let mut found_event = false;

                for event in response.events.iter() {
                    if event.ty == "loan_pool_withdrawn" {
                        found_event = true;
                        // Check event attributes here if needed
                    }
                }

                let mut found_attributes: Vec<String> = Vec::new();

                for attribute in response.attributes.iter() {
                    match attribute.key.as_str() {
                        "loan_pool_removed_by" => {
                            assert_eq!(attribute.value, info.sender.clone().to_string());
                            found_attributes.push(attribute.key.clone());
                        }
                        "action" => {
                            assert_eq!(attribute.value, "loan_pool_removed");
                            found_attributes.push(attribute.key.clone());
                        }
                        // Add more keys to check here
                        _ => (),
                    }
                }

                assert_eq!(
                    found_attributes.len(),
                    2,
                    "Did not find all required attributes"
                );

                assert_eq!(response.messages.len(), 2);

                assert!(found_event, "Failed to find loan_pool_withdrawn event");
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }
}
