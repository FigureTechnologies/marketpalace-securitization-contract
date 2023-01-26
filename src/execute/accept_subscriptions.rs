use cosmwasm_std::{Addr, Env, Response};
use provwasm_std::transfer_marker_coins;

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    state::{ACCEPTED, PENDING, SECURITY_TYPES},
};

pub fn accept_subscriptions(
    env: Env,
    deps: ProvDepsMut,
    subscriptions: Vec<Addr>,
) -> ProvTxResponse {
    for lp in subscriptions {
        // Update state PENDING -> ACCEPTED
        let subscription = PENDING.load(deps.storage, lp.clone())?;
        PENDING.remove(deps.storage, lp.clone());
        ACCEPTED.save(deps.storage, lp.clone(), &subscription)?;

        // Give the lp their commitment tokens
        // TODO We will want to make sure the contract has enough of the commitment tokens left
        for commitment in subscription.commitments {
            let security = SECURITY_TYPES.load(deps.storage, commitment.name)?;
            let contract_address = env.contract.address.clone();
            transfer_marker_coins(
                commitment.amount,
                security.get_commitment_name(&contract_address),
                lp.clone(),
                contract_address,
            )?;
        }
    }
    Ok(Response::new())
}
