use cosmwasm_std::{Addr, Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::state::update_batch_size,
    util::is_contract_admin::is_contract_admin,
};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr, batch_size: u128) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    update_batch_size(deps.storage, batch_size)?;
    Ok(Response::default()
        .add_attribute("action", "modify_batch_size")
        .add_attribute("new_batch_size", batch_size.to_string()))
}
