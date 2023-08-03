use cosmwasm_std::{Addr, DepsMut, Env, Event, MessageInfo, Response, Storage};
use provwasm_std::{AccessGrant, ProvenanceQuery};

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
use crate::storage::whitelist_contributors_store::get_whitelist_contributors;

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
// Define some dummy data for removed_permissions
    let empty_permissions: Vec<AccessGrant> = Vec::new();

    // Create a LoanPoolMarkerCollateral instance with some dummy values
    let collateral = LoanPoolMarkerCollateral::new(info.sender.clone(), marker_denom, 10, &empty_permissions);

    // Return the instance wrapped in a Result
    Ok(collateral)
}

