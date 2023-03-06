use cosmwasm_std::Response;

use crate::{
    core::aliases::{ProvDepsMut, ProvTxResponse},
    storage::state::update_batch_size,
};

pub fn handle(deps: ProvDepsMut, batch_size: u128) -> ProvTxResponse {
    update_batch_size(deps.storage, batch_size)?;
    Ok(Response::default()
        .add_attribute("action", "modify_batch_size")
        .add_attribute("new_batch_size", batch_size.to_string()))
}
