use cosmwasm_std::{Addr, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
};

pub fn handle(deps: ProvDepsMut, _sender: Addr, contracts: Vec<Addr>) -> ProvTxResponse {
    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    for contract in &contracts {
        storage::contract::remove(deps.storage, contract);
    }
    Ok(Response::default())
}
