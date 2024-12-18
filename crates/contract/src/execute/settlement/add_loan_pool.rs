use cosmwasm_std::OverflowOperation::Add;
use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response, Uint128,
};
use provwasm_std::types::provenance::marker::v1::Access::{Admin, Withdraw};
use provwasm_std::types::provenance::marker::v1::{AccessGrant, MarkerAccount, MarkerQuerier};
use result_extensions::ResultExtensions;
use std::str::FromStr;

use crate::core::collateral::{LoanPoolAdditionData, LoanPoolMarkerCollateral, LoanPoolMarkers};
use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    error::ContractError,
    security::ContributeLoanPools,
};
use crate::execute::settlement::marker_loan_pool_validation::validate_marker_for_loan_pool_add_remove;
use crate::storage::loan_pool_collateral::set;
use crate::storage::whitelist_contributors_store::get_whitelist_contributors;
use crate::util::provenance_utilities::{
    get_marker, get_marker_address, get_single_marker_coin_holding, query_total_supply,
    revoke_marker_access, Marker,
};

/// Handles loan pool additions.
///
/// This function accepts multiple loan pools and processes each loan pool. It first verifies
/// whether the sender is authorized i.e. whether the sender is in the `whitelist_contributors`.
/// It then creates a collateral for each loan pool, adds the collateral into the storage
/// (using the `set` function) and then updates the response with loan pool related messages
/// and events. If all the loan pools are successfully processed and collaterals are successfully
/// added to each one of them, the function then updates the `response` attributes and data
/// accordingly and returns it.
///
/// It is important to note that this function will fail if the sender is not in the whitelist of
/// contributors i.e. `whitelist_contributors` does not contain the sender.
///
/// # Arguments
/// * `deps` - A mutable reference to the provenance dependencies.
/// * `env` - The environment in which the contract is running.
/// * `info` - The information of the sender.
/// * `loan_pools` - Loan pools to be contributed to.
///
/// # Returns
/// * On Success - A `ProvTxResponse` containing updated response for each loan pool addition.
///   The response includes newly computed collaterals, messages and events.
///
/// * On Failure - An Err variant of `ProvTxResponse` which might contain a `ContractError` if:
///   - The sender is not in the whitelist contributors.
///   - There is any underlying failure with processing any of the loan pools.
pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    loan_pools: ContributeLoanPools,
) -> ProvTxResponse {
    // Load whitelist contributors from storage
    let whitelist_contributors = get_whitelist_contributors(deps.storage);

    // Check if sender is in the whitelist
    if !whitelist_contributors.contains(&info.sender) {
        return Err(ContractError::NotInWhitelist {});
    }

    // create empty response object
    let mut response = Response::new();

    let mut collaterals = Vec::new();

    for pool in loan_pools.markers {
        let LoanPoolAdditionData {
            collateral,
            messages,
        } = create_marker_pool_collateral(&deps, &info, &env, pool.clone())?;
        //insert the collateral
        set(deps.storage, &collateral)?;
        collaterals.push(collateral);
        // Add messages and event in a chained manner
        response = response.add_messages(messages).add_event(
            Event::new("loan_pool_added").add_attribute("marker_address", pool.to_string()),
        );
    }

    // Add added_by attribute only if loan_pool_added event is added
    if response
        .events
        .iter()
        .any(|event| event.ty == "loan_pool_added")
    {
        response = response.add_attribute("loan_pool_added_by", info.sender);
        response = response.add_attribute("action", "loan_pool_added");
    }
    // Set response data to collaterals vector
    response = response.set_data(to_json_binary(&LoanPoolMarkers::new(collaterals))?);

    Ok(response)
}

/// Creates the collateral for a specified marker pool. This function
/// gets and validates the marker, computes the total supply, checks if
/// the caller of the function has admin rights to the marker, checks
/// if the supply of the marker is completely held by the marker and
/// upon success, it fetches messages to revoke marker permissions and updates the response
/// with collateral data and messages regarding admin rights
///
/// Parameters:
/// * `deps`: the dependency object giving access to contract's storage, APIs for making queries on blockchain, etc.
/// * `info`: the information of the message
/// * `env`: the environment information where the contract is executed.
/// * `marker_denom`: the denom of the marker.
///
/// Returns:
/// * `Result<collateral::LoanPoolAdditionData, ContractError>`: Result object which on success contains custom LoanPoolAddtionData value
///   or a custom ContractError enumeration, which represents an error.
///
/// # Errors
/// * if unable to fetch the marker by denom.
/// * if unable to fetch the marker's total supply.
/// * if unable to validate the marker for addition to loan pool
/// * if unable to get messages to revoke the marker's permissions
fn create_marker_pool_collateral(
    deps: &DepsMut,
    info: &MessageInfo,
    env: &Env,
    marker_denom: String,
) -> Result<LoanPoolAdditionData, ContractError> {
    // get marker
    let querier = MarkerQuerier::new(&deps.querier);
    let marker_res = get_marker(marker_denom.clone(), &querier);
    let marker = match marker_res {
        Ok(m) => m,
        Err(e) => {
            return Err(ContractError::InvalidMarker {
                message: format!("Unable to get marker by denom: {}", e),
            });
        }
    };

    // each marker has a supply
    let supply =
        query_total_supply(deps, marker_denom).map_err(|e| ContractError::InvalidMarker {
            message: format!("Error when querying total supply: {}", e),
        })?;

    // validate that the loan pool marker can be added to the securitization.
    // This involves
    // 1. Checking that the sender of the message has ADMIN rights to the marker
    // 2. The supply of the marker is completely held by the marker.
    validate_marker_for_loan_pool_add_remove(
        deps,
        &marker,
        // New loan pool contribution should verify that the sender owns the marker, and then revoke its permissions
        &info.sender,
        &env.contract.address,
        &[Admin, Withdraw],
        supply,
    )?;

    let messages = get_marker_permission_revoke_messages(&marker, &env.contract.address)?;
    let marker_address = get_marker_address(marker.base_account.clone())?;
    let share_count = Uint128::from_str(
        get_single_marker_coin_holding(&deps, &marker.clone())?
            .amount
            .as_str(),
    )?
    .u128();

    LoanPoolAdditionData {
        collateral: LoanPoolMarkerCollateral::new(
            Addr::unchecked(marker_address),
            &marker.denom,
            share_count,
            info.sender.to_owned(),
            marker
                .clone()
                .access_control
                .into_iter()
                .filter(|perm| Addr::unchecked(perm.address.clone()) != env.contract.address)
                .collect::<Vec<AccessGrant>>(),
        ),
        messages,
    }
    .to_ok()
}
/// A helper function to construct messages required to revoke marker permissions
///
/// The function filters through access grants on the marker and prepares revoke messages
/// for those permissions where the granted access is not to this contract. These messages
/// can then be executed on chain to actually revoke the access.
///
/// Parameters:
/// * `marker`: the marker object from which permissions are to be revoked
/// * `contract_address`: the address of this contract
///
/// Returns:
/// * `Result<Vec<CosmosMsg>, ContractError>`: A result object containing either a vector of messages to revoke access
///   or a custom ContractError enumeration, which represents an error.
fn get_marker_permission_revoke_messages(
    marker: &MarkerAccount,
    contract_address: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    for permission in marker
        .access_control
        .iter()
        .filter(|perm| &Addr::unchecked(perm.address.clone()) != contract_address)
    {
        messages.push(revoke_marker_access(
            &marker.denom,
            Addr::unchecked(permission.address.clone()),
        )?);
    }
    messages.to_ok()
}

#[cfg(test)]
mod tests {
    use crate::core::collateral::{
        AccessGrantSerializable, LoanPoolMarkerCollateral, LoanPoolMarkers,
    };
    use crate::core::error::ContractError;
    use crate::core::security::ContributeLoanPools;
    use crate::execute::settlement::add_loan_pool::{
        create_marker_pool_collateral, get_marker_permission_revoke_messages,
        handle as add_loanpool_handle,
    };
    use crate::execute::settlement::whitelist_loanpool_contributors::handle as whitelist_loanpool_handle;
    use crate::util::mock_marker::{MockMarker, DEFAULT_MARKER_ADDRESS, DEFAULT_MARKER_DENOM};
    use crate::util::testing::instantiate_contract;
    use cosmwasm_std::testing::{message_info, mock_env};
    use cosmwasm_std::CosmosMsg::Custom;
    use cosmwasm_std::ReplyOn::Never;
    use cosmwasm_std::{
        coins, from_json, to_json_binary, Addr, AnyMsg, Binary, ContractResult, Empty, Event,
        Response, SubMsg, SystemResult,
    };
    use provwasm_mocks::{
        mock_provenance_dependencies, mock_provenance_dependencies_with_custom_querier,
    };
    use provwasm_std::shim::Any;
    use provwasm_std::types::cosmos::base::v1beta1::Coin;
    use provwasm_std::types::provenance::marker::v1::{
        AccessGrant, Balance, MarkerQuerier, QueryHoldingRequest, QueryHoldingResponse,
        QueryMarkerRequest, QueryMarkerResponse,
    };

    #[test]
    fn test_coin_trade_with_valid_data() {
        let mut response: Response<Empty> = Response::new();
        response = response
            .add_event(Event::new("loanpool_added").add_attribute("marker_address", "addr1"));
        response = response
            .add_event(Event::new("loanpool_added").add_attribute("marker_address", "addr2"));

        // Now the response object contains two separate events with the name "loanpool_added."
        assert_eq!(response.events.len(), 2);
    }

    #[test]
    fn test_handle_not_in_whitelist() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();
        // deps.querier.with_markers(vec![marker]);
        let env = mock_env();
        let info = message_info(&Addr::unchecked("contributor"), &[]);
        //
        // Create a loan pool
        let loan_pools = ContributeLoanPools {
            markers: vec![marker_denom],
        };
        // Call the handle function
        let result = add_loanpool_handle(deps.as_mut(), env, info, loan_pools);
        // Assert that the result is an error
        assert!(result.is_err());
        //
        // Assert that the error is a ContractError::NotInWhitelist
        match result.unwrap_err() {
            ContractError::NotInWhitelist {} => (),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_handle_in_whitelist() {
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
            markers: vec![marker_denom.clone()],
        };

        let expected_collaterals = vec![LoanPoolMarkerCollateral {
            marker_address: marker.address.clone(),
            marker_denom,
            share_count: marker.total_supply,
            original_contributor: info.sender.to_owned(),
            removed_permissions: if let Some(first_permission) = marker.permissions.first() {
                vec![AccessGrantSerializable::from(first_permission.clone())]
            } else {
                vec![]
            },
        }];
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

                assert_eq!(response.events.len(), 1);
                assert_eq!(response.attributes.len(), 2);
                for event in response.events.iter() {
                    if event.ty == "loan_pool_added" {
                        found_event = true;
                    }
                }
                let mut found_attributes: Vec<String> = Vec::new();

                for attribute in response.attributes.iter() {
                    match attribute.key.as_str() {
                        "loan_pool_added_by" => {
                            assert_eq!(attribute.value, info.sender.clone().to_string());
                            found_attributes.push(attribute.key.clone());
                        }
                        "action" => {
                            assert_eq!(attribute.value, "loan_pool_added");
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

                assert_eq!(response.messages.len(), 1);

                // let expected_msg1 = SubMsg {
                //     id: 0,
                //     msg: Custom(AnyMsg {
                //         route: marker_route,
                //         params: Marker(RevokeMarkerAccess {
                //             denom: "markerdenom".parse().unwrap(),
                //             address: Addr::unchecked("contributor".to_string()),
                //         }),
                //         version: "2.0.0".parse().unwrap(),
                //     }),
                //     gas_limit: None,
                //     reply_on: Never,
                // };
                //
                // assert_eq!(response.messages[0], expected_msg1);
                assert!(found_event, "Failed to find loan_pool_added event");
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_create_marker_pool_collateral_error_invalid_marker() {
        let mut deps = mock_provenance_dependencies();

        /* Create the necessary mocked objects. You would need to replace "someAddress",
        "someMarkerDenom", and "someEnv" with corresponding valid objects */
        let info = message_info(&Addr::unchecked("someAddress"), &[]);
        let env = mock_env();

        // use a string that doesn't correspond to an existing marker
        let marker_denom = String::from("nonExistentMarkerDenom");

        let result = create_marker_pool_collateral(&deps.as_mut(), &info, &env, marker_denom);

        // Assert that the result is an error because the marker doesn't exist
        assert!(result.is_err());

        match result.unwrap_err() {
            ContractError::InvalidMarker { .. } => (),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_get_marker_permission_revoke_messages() {
        let marker = MockMarker::new_owned_marker("markerOwner").to_marker_account();
        let contract_address = Addr::unchecked("contractAddress");

        let result = get_marker_permission_revoke_messages(&marker, &contract_address);

        // Assert that the result is ok
        assert!(result.is_ok());
        match result.ok() {
            Some(revoke_messages) => {
                // Assert that the messages to revoke access are as expected
                // This depends on the specifics of your implementation
                assert_eq!(revoke_messages.len(), marker.access_control.len());
            }
            None => panic!("Expected some revoke messages, got None"),
        }
    }

    #[test]
    fn test_get_marker_permission_revoke_messages_contract_addr_there() {
        let marker = MockMarker::new_owned_marker("markerOwner").to_marker_account();
        let contract_address = Addr::unchecked("cosmos2contract");

        let result = get_marker_permission_revoke_messages(&marker, &contract_address);

        // Assert that the result is ok
        assert!(result.is_ok());
        match result.ok() {
            Some(revoke_messages) => {
                // Assert that the messages to revoke access are as expected
                // This depends on the specifics of your implementation
                assert_eq!(revoke_messages.len(), marker.access_control.len() - 1);
            }
            None => panic!("Expected some revoke messages, got None"),
        }
    }
}
