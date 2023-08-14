use cosmwasm_std::{to_binary, Addr, DepsMut, Env, Event, MessageInfo, Response};
use provwasm_std::{ProvenanceQuerier, ProvenanceQuery};

use crate::core::collateral::{LoanPoolMarkers, LoanPoolRemovalData};
use crate::core::security::WithdrawLoanPools;
use crate::storage::loan_pool_collateral::{get, remove};
use crate::util::provenance_utilities::release_marker_from_contract;
use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::state::{self},
};

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

    // Validate addresses and fetch removal data
    let removal_data: Vec<_> = loan_pools
        .markers
        .iter()
        .map(|pool| {
            let address =
                deps.api
                    .addr_validate(pool)
                    .map_err(|_| ContractError::InvalidAddress {
                        message: pool.clone(),
                    })?;
            let removal_data = withdraw_marker_pool_collateral(&deps, &env, address.to_owned())?;
            Ok((address, removal_data))
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

    // Add added_by attribute only if loan_pool_added event is added
    if response
        .events
        .iter()
        .any(|event| event.ty == "loan_pool_withdrawn")
    {
        response = response.add_attribute("removed_by", info.sender.clone());
    }
    // Set response data to collaterals vector
    response = response.set_data(to_binary(&LoanPoolMarkers::new(collaterals))?);

    Ok(response)
}

fn withdraw_marker_pool_collateral(
    deps: &DepsMut<ProvenanceQuery>,
    env: &Env,
    marker_address: Addr,
) -> Result<LoanPoolRemovalData, ContractError> {
    // get marker
    let marker =
        ProvenanceQuerier::new(&deps.querier).get_marker_by_address(marker_address.clone())?;
    let collateral = get(deps.storage, marker_address.clone())?;
    let messages = release_marker_from_contract(
        marker.denom,
        &env.contract.address,
        &collateral.removed_permissions,
    )?;
    Ok(LoanPoolRemovalData {
        collateral,
        messages,
    })
}

#[cfg(test)]
mod tests {
    use crate::core::error::ContractError;
    use crate::core::security::WithdrawLoanPools;
    use crate::execute::settlement::withdraw_loan_pool::handle;
    use crate::util::mock_marker::MockMarker;
    use crate::util::testing::instantiate_contract;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_handle_should_fail_when_sender_is_not_gp() {
        let mut deps = mock_dependencies(&[]);
        let marker = MockMarker::new_owned_marker("contributor");
        let marker_denom = marker.denom.clone();
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let env = mock_env();
        let info = mock_info("someone", &[]);
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
}
