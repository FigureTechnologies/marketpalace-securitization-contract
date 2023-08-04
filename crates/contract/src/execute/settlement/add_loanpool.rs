use cosmwasm_std::{Addr, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response, Storage};
use provwasm_std::{AccessGrant, Marker, MarkerAccess, ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery, revoke_marker_access};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        security::{AcceptedCommitment, SecurityCommitment, ContributeLoanPools},
    },
    storage::{
        commits::{self},
        paid_in_capital::{self},
        remaining_securities,
        state::{self},
    },
    util::settlement::timestamp_is_expired,
};
use crate::core::collateral::{LoanPoolAdditionData, LoanPoolMarkerCollateral};
use crate::execute::settlement::extensions::ResultExtensions;
use crate::execute::settlement::marker_loan_pool_validation::validate_marker_for_loan_pool_add_remove;
use crate::storage::loan_pool_collateral::set;
use crate::storage::whitelist_contributors_store::get_whitelist_contributors;
use crate::util::provenance_utilities::{get_single_marker_coin_holding, query_total_supply};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    loan_pools: ContributeLoanPools,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;

    // Load whitelist contributors from storage
    let whitelist_contributors = get_whitelist_contributors(deps.storage)?;

    // Check if sender is in the whitelist
    if !whitelist_contributors.contains(&info.sender) {
        return Err(ContractError::NotInWhitelist {});
    }

    let mut response = Response::new()
        .add_attribute("added_by", info.sender.clone());
    for pool in loan_pools.markers {
        let LoanPoolAdditionData {
            collateral,
            messages
        } = create_marker_pool_collateral(&deps, &info, &env, pool.clone()).unwrap();
        //inset the collateral
        set(deps.storage,&collateral)?;

        // Add messages and event in a chained manner
        response = response.add_messages(messages)
            .add_event(Event::new("loanpool_added").add_attribute("marker_address", pool.to_string()));
    }

    Ok(response)
}

fn accept_loan_pool(
    storage: &mut dyn Storage,
    marker_address: Addr,
) -> Result<(), ContractError> {
    Ok(())
}


fn create_marker_pool_collateral(
    deps: &DepsMut<ProvenanceQuery>,
    info: &MessageInfo,
    env: &Env,
    marker_denom: String,
) -> Result<LoanPoolAdditionData, ContractError> {

    // get marker
    let marker =
        ProvenanceQuerier::new(&deps.querier).get_marker_by_denom(&marker_denom)?;

    // each marker has a supply
    let supply = query_total_supply(deps, &*marker_denom)
        .map_err(|e| ContractError::InvalidMarker {
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
            marker.permissions
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
    use cosmwasm_std::{Empty, Event, Response};

    #[test]
    fn test_coin_trade_with_valid_data() {
        let mut response: Response<Empty>  = Response::new();
        response = response.add_event(Event::new("loanpool_added").add_attribute("marker_address", "addr1"));
        response = response.add_event(Event::new("loanpool_added").add_attribute("marker_address", "addr2"));

// Now the response object contains two separate events with the name "loanpool_added."
        assert_eq!(response.events.len(), 2);
    }

}