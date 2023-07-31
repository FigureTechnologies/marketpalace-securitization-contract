use cosmwasm_std::{Addr, Env, Event, Response, Storage};

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

use super::commitment::{Commitment, CommitmentState};

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    loanPools: ContributeLoanPools,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        return Err(crate::core::error::ContractError::Unauthorized {});
    }

    if timestamp_is_expired(deps.storage, &env.block.time)? {
        return Err(crate::core::error::ContractError::SettlmentExpired {});
    }

    let mut response = Response::new()
        .add_attribute("action", "accept_commitments")
        .add_attribute("gp", state.gp);
    for pool in loanPools.markers {
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


