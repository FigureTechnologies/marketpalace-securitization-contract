use cosmwasm_std::{Addr, Response};

use crate::{
    core::aliases::{ProvDepsMut, ProvTxResponse},
    storage,
};

pub fn handle(deps: ProvDepsMut, _sender: Addr, contracts: Vec<Addr>) -> ProvTxResponse {
    if storage::state::is_migrating(deps.storage)? {
        // Throw an error
    }

    for contract in &contracts {
        storage::contract::add(deps.storage, contract)?;
    }
    Ok(Response::default())
}
