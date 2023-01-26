use cosmwasm_std::{Addr, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::SecurityCommitment,
        state::PENDING,
    },
    subscription::Subscription,
};

pub fn propose_subscription(
    deps: ProvDepsMut,
    lp: Addr,
    commitments: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    // TODO We probably want to validate the minimums

    let subscription = Subscription::new(lp.clone(), commitments);

    // Maybe we want to verify that they actually have the funds they are committing?

    PENDING.save(deps.storage, lp, &subscription)?;
    Ok(Response::new())
}
