use cosmwasm_std::{Addr, Response};

use crate::{
    commitment::Commitment,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        security::SecurityCommitment,
        state::COMMITS,
    },
};

pub fn handle(deps: ProvDepsMut, lp: Addr, commitments: Vec<SecurityCommitment>) -> ProvTxResponse {
    // TODO We probably want to validate the minimums

    let commitment = Commitment::new(lp.clone(), commitments);

    COMMITS.save(deps.storage, lp, &commitment)?;
    Ok(Response::new())
}
