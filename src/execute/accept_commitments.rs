use cosmwasm_std::{Addr, Env, Response};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    state::{ACCEPTED, COMMITS, PENDING},
};

pub fn accept_commitments(_env: Env, deps: ProvDepsMut, commitments: Vec<Addr>) -> ProvTxResponse {
    for lp in commitments {
        // Update state PENDING -> ACCEPTED
        let commitment = PENDING.load(deps.storage, lp.clone())?;
        PENDING.remove(deps.storage, lp.clone());
        ACCEPTED.save(deps.storage, lp.clone(), &commitment)?;

        // Create a new commit that can be updated when the LP deposits
        let mut commit = commitment.clone();
        commit.clear();
        COMMITS.save(deps.storage, lp.clone(), &commit)?;
    }
    Ok(Response::new())
}
