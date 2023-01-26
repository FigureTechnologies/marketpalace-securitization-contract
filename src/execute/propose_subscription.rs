use cosmwasm_std::{Addr, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        state::PENDING,
    },
    subscription::Subscription,
};

pub fn propose_subscription(
    deps: ProvDepsMut,
    lp: &Addr,
    admin: String,
    code_id: u64,
    commitment_denom: &str,
    recovery: &Addr,
    initial_commitment: Option<u64>,
) -> ProvTxResponse {
    let subscription = Subscription::new(
        recovery.clone(),
        lp.clone(),
        commitment_denom.to_string(),
        initial_commitment,
    );

    PENDING.save(deps.storage, lp.clone(), &true)?;

    Ok(Response::new())
}
