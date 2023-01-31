use cosmwasm_std::{Addr, Response};

use crate::{
    commitment::CommitmentState,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        state::{COMMITS, PAID_IN_CAPITAL, STATE},
    },
};

pub fn handle(deps: ProvDepsMut, sender: Addr, commitments: Vec<Addr>) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
    if sender != state.gp {
        // TODO Throw an error
    }

    for lp in commitments {
        let mut commitment = COMMITS.load(deps.storage, lp.clone())?;

        if commitment.state != CommitmentState::PENDING {
            // TODO Throw an error
        }

        commitment.state = CommitmentState::ACCEPTED;
        COMMITS.save(deps.storage, lp.clone(), &commitment)?;

        // Create a new commit that can be updated when the LP deposits
        let mut commit = commitment.clone();
        commit.clear_amounts();
        PAID_IN_CAPITAL.save(deps.storage, lp.clone(), &commit.commitments)?;
    }
    Ok(Response::new())
}
