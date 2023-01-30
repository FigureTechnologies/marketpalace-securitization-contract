use cosmwasm_std::{Addr, Response};

use crate::{
    commitment::Commitment,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::SecurityCommitment,
        state::COMMITS,
    },
};

pub fn propose_commitment(
    deps: ProvDepsMut,
    lp: Addr,
    commitments: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    // TODO We probably want to validate the minimums

    let commitment = Commitment::new(lp.clone(), commitments);

    // Maybe we want to verify that they actually have the funds they are committing?

    COMMITS.save(deps.storage, lp, &commitment)?;
    Ok(Response::new())
}
