use cosmwasm_std::{Addr, Response};

use crate::{
    core::aliases::{ProvDepsMut, ProvTxResponse},
    storage,
};

pub fn handle(deps: ProvDepsMut, _sender: Addr, contracts: Vec<Addr>) -> ProvTxResponse {
    for contract in contracts {
        storage::contract::remove(deps.storage, contract);
    }
    Ok(Response::default())
}
