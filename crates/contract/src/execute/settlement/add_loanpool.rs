use cosmwasm_std::{Addr, DepsMut, Env, Event, MessageInfo, Response, Storage};
use provwasm_std::{AccessGrant, MarkerAccess, ProvenanceQuerier, ProvenanceQuery};

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
use crate::core::collateral::LoanPoolMarkerCollateral;
use crate::execute::settlement::marker_loan_pool_validation::validate_marker_for_loan_pool_add_remove;
use crate::storage::whitelist_contributors_store::get_whitelist_contributors;
use crate::util::provenance_utilities::query_total_supply;

use super::commitment::{Commitment, CommitmentState};

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    loan_pools: ContributeLoanPools,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;

    // Load whitelist contributors from storage
    let whitelist_contributors = get_whitelist_contributors(deps.storage)?;

    // Check if sender is in the whitelist
    if !whitelist_contributors.contains(&sender) {
        return Err(ContractError::NotInWhitelist {});
    }

    let mut response = Response::new()
        .add_attribute("action", "accept_commitments")
        .add_attribute("gp", state.gp);
    for pool in loan_pools.markers {
        // accept_commitment(deps.storage, commitment.clone())?;
        // add the marker, change owner, escrow the account
        response = response.add_event(Event::new("loanpool_added").add_attribute("marker_address", pool.to_string()));
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
) -> Result<LoanPoolMarkerCollateral, ContractError> {
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
// Define some dummy data for removed_permissions
    let empty_permissions: Vec<AccessGrant> = Vec::new();

    // Create a LoanPoolMarkerCollateral instance with some dummy values
    let collateral = LoanPoolMarkerCollateral::new(info.sender.clone(), marker_denom, 10, &empty_permissions);

    // Return the instance wrapped in a Result
    Ok(collateral)
}


