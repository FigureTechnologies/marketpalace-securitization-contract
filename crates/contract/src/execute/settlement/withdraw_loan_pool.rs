use cosmwasm_std::{Addr, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response, Storage, to_binary};
use provwasm_std::{AccessGrant, grant_marker_access, Marker, MarkerAccess, ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery, revoke_marker_access};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::{
        state::{self},
    },
};
use crate::core::collateral::{LoanPoolRemovalData, LoanPoolMarkerCollateral, LoanPoolMarkers};
use crate::core::security::WithdrawLoanPools;
use crate::execute::settlement::extensions::ResultExtensions;
use crate::storage::loan_pool_collateral::{get, remove};
use crate::util::provenance_utilities::release_marker_from_contract;


pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    loan_pools: WithdrawLoanPools,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;

    // the gp can only release the pool
    if info.sender != state.gp  {
        return Err(ContractError::Unauthorized {});
    }

    // create empty response object
    let mut response = Response::new();

    let mut collaterals = Vec::new();

    // Validate addresses and fetch removal data
    let removal_data: Vec<_> = loan_pools.markers.iter()
        .map(|pool| {
            let address = deps.api.addr_validate(pool)
                .map_err(|_| ContractError::InvalidAddress { message: pool.clone() })?;
            let removal_data = withdraw_marker_pool_collateral(&deps, &info, &env, pool.clone())?;
            Ok((address, removal_data))
        })
        .collect::<Result<_, ContractError>>()?;

    // Modify state
    for (address, LoanPoolRemovalData { collateral, messages }) in removal_data {

        remove(deps.storage, &collateral)?;

        // store each collateral in collaterals vector
        collaterals.push(collateral);

        response = response.add_messages(messages)
            .add_event(Event::new("loan_pool_withdrawn")
            .add_attribute("marker_address", address.to_string()));
    }


    // Add added_by attribute only if loan_pool_added event is added
    if response.events.iter().any(|event| event.ty == "loan_pool_withdrawn") {
        response = response.add_attribute("removed_by", info.sender.clone());
    }
    // Set response data to collaterals vector
    response = response.set_data(to_binary(&LoanPoolMarkers::new(collaterals))?);

    Ok(response)
}

fn withdraw_marker_pool_collateral(
    deps: &DepsMut<ProvenanceQuery>,
    info: &MessageInfo,
    env: &Env,
    marker_address: Addr,
) -> Result<LoanPoolRemovalData, ContractError> {
    // get marker
    let marker =
        ProvenanceQuerier::new(&deps.querier).get_marker_by_address(marker_address.clone())?;
    let collateral = get(deps.storage, marker_address.clone())?;
    let messages = release_marker_from_contract(&marker, &env.contract.address, &collateral.removed_permissions)?;
    Ok(LoanPoolRemovalData {
        collateral,
        messages
    })
}



#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Empty, Event, from_binary, Response};
    use cosmwasm_std::testing::{mock_env, mock_info};
    use provwasm_mocks::mock_dependencies;
    use crate::core::collateral::{LoanPoolMarkerCollateral, LoanPoolMarkers};
    use crate::core::error::ContractError;
    use crate::core::security::ContributeLoanPools;
    use crate::execute::settlement::add_loan_pool::handle as add_loanpool_handle;
    use crate::execute::settlement::whitelist_loanpool_contributors::handle as whitelist_loanpool_handle;
    use crate::util::mock_marker::MockMarker;
    use crate::util::testing::instantiate_contract;

    #[test]
    fn test_coin_trade_with_valid_data() {
        let mut response: Response<Empty> = Response::new();
        response = response.add_event(Event::new("loanpool_added").add_attribute("marker_address", "addr1"));
        response = response.add_event(Event::new("loanpool_added").add_attribute("marker_address", "addr2"));

// Now the response object contains two separate events with the name "loanpool_added."
        assert_eq!(response.events.len(), 2);
    }

    #[test]
    fn test_handle_not_in_whitelist() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();
        deps.querier
            .with_markers(vec![marker]);
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
        deps.querier
            .with_markers(vec![marker.clone()]);
        let env = mock_env();
        let env_white_list = env.clone();
        let env_loan_pool = env.clone();
        let info = mock_info("contributor", &[]);
        let info_white_list = mock_info("gp", &[]);
        let info_loan_pool = mock_info("gp", &[]);
        let addr_contributor = Addr::unchecked("contributor");
        let white_list_addr = vec![addr_contributor.clone()];
        let whitelist_result = whitelist_loanpool_handle(deps.as_mut(),  info_white_list.sender, white_list_addr);
        assert!(whitelist_result.is_ok());
        match whitelist_result {
            Ok(response) => {
                let mut found_action = false;
                let mut found_address = false;

                for attribute in response.attributes.iter() {
                    if attribute.key == "action" {
                        assert_eq!(attribute.value, "whitelist_added");
                        found_action = true;
                    } else if attribute.key == "address_whitelisted" {
                        // Verify if the addresses are correct
                        let whitelisted_addresses: Vec<&str> = attribute.value.split(",").collect();
                        assert_eq!(whitelisted_addresses, vec!["contributor"]);
                        found_address = true;
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
        let loan_pool_result = add_loanpool_handle(deps.as_mut(), env_loan_pool, info.clone(), loan_pools);
        // Assert that the result is an error
        assert!(loan_pool_result.is_ok());
        match loan_pool_result {
            Ok(response) => {
                // Checking response data
                let loan_pool_markers: LoanPoolMarkers = from_binary(&response.data.unwrap()).unwrap();
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

    fn do_valid_marker_add_pool() {}
}