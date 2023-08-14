use cosmwasm_std::{to_binary, Addr, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response};
use provwasm_std::{
    revoke_marker_access, AccessGrant, Marker, MarkerAccess, ProvenanceMsg, ProvenanceQuerier,
    ProvenanceQuery,
};

use crate::core::collateral::{LoanPoolAdditionData, LoanPoolMarkerCollateral, LoanPoolMarkers};
use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    error::ContractError,
    security::ContributeLoanPools,
};
use crate::execute::settlement::extensions::ResultExtensions;
use crate::execute::settlement::marker_loan_pool_validation::validate_marker_for_loan_pool_add_remove;
use crate::storage::loan_pool_collateral::set;
use crate::storage::whitelist_contributors_store::get_whitelist_contributors;
use crate::util::provenance_utilities::{get_single_marker_coin_holding, query_total_supply};

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    loan_pools: ContributeLoanPools,
) -> ProvTxResponse {
    // Load whitelist contributors from storage
    let whitelist_contributors = get_whitelist_contributors(deps.storage)?;

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
        response = response.add_attribute("added_by", info.sender);
    }
    // Set response data to collaterals vector
    response = response.set_data(to_binary(&LoanPoolMarkers::new(collaterals))?);

    Ok(response)
}

fn create_marker_pool_collateral(
    deps: &DepsMut<ProvenanceQuery>,
    info: &MessageInfo,
    env: &Env,
    marker_denom: String,
) -> Result<LoanPoolAdditionData, ContractError> {
    // get marker
    let marker_res = ProvenanceQuerier::new(&deps.querier).get_marker_by_denom(&marker_denom);
    let marker = match marker_res {
        Ok(m) => m,
        Err(e) => {
            return Err(ContractError::InvalidMarker {
                message: format!("Unable to get marker by denom: {}", e),
            })
        }
    };

    // each marker has a supply
    let supply =
        query_total_supply(deps, &*marker_denom).map_err(|e| ContractError::InvalidMarker {
            message: format!("Error when querying total supply: {}", e),
        })?;

    // validate that the loan pool marker can be added to the securitization.
    // This involves
    // 1. Checking that the sender of the message has ADMIN rights to the marker
    // 2. The supply of the marker is completely held by the marker.
    validate_marker_for_loan_pool_add_remove(
        &marker,
        // New asks should verify that the sender owns the marker, and then revoke its permissions
        Some(&info.sender),
        &env.contract.address,
        &[MarkerAccess::Admin, MarkerAccess::Withdraw],
        supply,
    )?;

    let messages = get_marker_permission_revoke_messages(&marker, &env.contract.address)?;

    LoanPoolAdditionData {
        collateral: LoanPoolMarkerCollateral::new(
            marker.address.clone(),
            &marker.denom,
            get_single_marker_coin_holding(&marker)?.amount.u128(),
            marker
                .permissions
                .into_iter()
                .filter(|perm| perm.address != env.contract.address)
                .collect::<Vec<AccessGrant>>(),
        ),
        messages,
    }
    .to_ok()
}

fn get_marker_permission_revoke_messages(
    marker: &Marker,
    contract_address: &Addr,
) -> Result<Vec<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let mut messages: Vec<CosmosMsg<ProvenanceMsg>> = vec![];
    for permission in marker
        .permissions
        .iter()
        .filter(|perm| &perm.address != contract_address)
    {
        messages.push(revoke_marker_access(
            &marker.denom,
            permission.address.clone(),
        )?);
    }
    messages.to_ok()
}

#[cfg(test)]
mod tests {
    use crate::core::collateral::{LoanPoolMarkerCollateral, LoanPoolMarkers};
    use crate::core::error::ContractError;
    use crate::core::security::ContributeLoanPools;
    use crate::execute::settlement::add_loan_pool::{
        create_marker_pool_collateral, get_marker_permission_revoke_messages,
        handle as add_loanpool_handle,
    };
    use crate::execute::settlement::whitelist_loanpool_contributors::handle as whitelist_loanpool_handle;
    use crate::util::mock_marker::{MockMarker, DEFAULT_MARKER_ADDRESS, DEFAULT_MARKER_DENOM};
    use crate::util::testing::instantiate_contract;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr, BankMsg, CosmosMsg, Empty, Event, Response};
    use provwasm_mocks::mock_dependencies;

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
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();
        deps.querier.with_markers(vec![marker]);
        let env = mock_env();
        let info = mock_info("contributor", &[]);
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
            share_count: marker.total_supply.atomics(),
            removed_permissions: if let Some(first_permission) = marker.permissions.first() {
                vec![first_permission.clone()]
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
                    from_binary(&response.data.unwrap()).unwrap();
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
                    if attribute.key == "added_by" {
                        assert_eq!(attribute.value, info.sender.clone());
                        found_attribute = true;
                    }
                }

                assert!(found_event, "Failed to find loan_pool_added event");
                assert!(found_attribute, "Failed to find added_by attribute");
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_create_marker_pool_collateral_error_invalid_marker() {
        let mut deps = mock_dependencies(&[]);

        /* Create the necessary mocked objects. You would need to replace "someAddress",
        "someMarkerDenom", and "someEnv" with corresponding valid objects */
        let info = mock_info("someAddress", &[]);
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
        let marker = MockMarker::new_owned_marker("markerOwner");
        let contract_address = Addr::unchecked("contractAddress");

        let result = get_marker_permission_revoke_messages(&marker, &contract_address);

        // Assert that the result is ok
        assert!(result.is_ok());
        match result.ok() {
            Some(revoke_messages) => {
                // Assert that the messages to revoke access are as expected
                // This depends on the specifics of your implementation
                assert_eq!(revoke_messages.len(), marker.permissions.len());
            }
            None => panic!("Expected some revoke messages, got None"),
        }
    }

    #[test]
    fn test_get_marker_permission_revoke_messages_contract_addr_there() {
        let marker = MockMarker::new_owned_marker("markerOwner");
        let contract_address = Addr::unchecked("cosmos2contract");

        let result = get_marker_permission_revoke_messages(&marker, &contract_address);

        // Assert that the result is ok
        assert!(result.is_ok());
        match result.ok() {
            Some(revoke_messages) => {
                // Assert that the messages to revoke access are as expected
                // This depends on the specifics of your implementation
                assert_eq!(revoke_messages.len(), marker.permissions.len() - 1);
            }
            None => panic!("Expected some revoke messages, got None"),
        }
    }

    #[test]
    fn test_handle_in_whitelist_validation_fail() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker_supply_variable("contributor");
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

        // transfer some value out of the marker
        let addr = "addr1".to_string();
        let balance = coins(10, DEFAULT_MARKER_DENOM);

        // Update the balance for the given address
        let old_balance = deps
            .querier
            .base
            .update_balance(DEFAULT_MARKER_ADDRESS, balance.clone());

        // Create a loan pool
        let loan_pools = ContributeLoanPools {
            markers: vec![marker_denom.clone()],
        };

        let expected_collaterals = vec![LoanPoolMarkerCollateral {
            marker_address: marker.address.clone(),
            marker_denom,
            share_count: marker.total_supply.atomics(),
            removed_permissions: if let Some(first_permission) = marker.permissions.first() {
                vec![first_permission.clone()]
            } else {
                vec![]
            },
        }];
        // Call the handle function
        let loan_pool_result =
            add_loanpool_handle(deps.as_mut(), env.to_owned(), info.clone(), loan_pools);
        // Assert that the result is an error
        assert!(loan_pool_result.is_err());
        match loan_pool_result {
            Ok(response) => {
                // Checking response data
                let loan_pool_markers: LoanPoolMarkers =
                    from_binary(&response.data.unwrap()).unwrap();
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
                    if attribute.key == "added_by" {
                        assert_eq!(attribute.value, info.sender.clone());
                        found_attribute = true;
                    }
                }

                assert!(found_event, "Failed to find loan_pool_added event");
                assert!(found_attribute, "Failed to find added_by attribute");
            }
            Err(e) => match e {
                ContractError::InvalidMarker { .. } => (), // continue
                unexpected_error => panic!("Error: {:?}", unexpected_error),
            },
        }
    }
}
