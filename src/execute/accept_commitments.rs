use cosmwasm_std::{Addr, Env, Response};

use crate::{
    commitment::CommitmentState,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        state::{COMMITS, PAID_IN_CAPITAL},
    },
};

pub fn accept_commitments(_env: Env, deps: ProvDepsMut, commitments: Vec<Addr>) -> ProvTxResponse {
    for lp in commitments {
        let mut commitment = COMMITS.load(deps.storage, lp.clone())?;

        if commitment.state != CommitmentState::PENDING {
            // TODO
            // Throw an error
        }

        commitment.state = CommitmentState::ACCEPTED;
        COMMITS.save(deps.storage, lp.clone(), &commitment)?;

        // Create a new commit that can be updated when the LP deposits
        let mut commit = commitment.clone();
        commit.clear();
        PAID_IN_CAPITAL.save(deps.storage, lp.clone(), &commit.commitments)?;
    }
    Ok(Response::new())
}
