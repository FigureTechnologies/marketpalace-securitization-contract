use cosmwasm_std::{Addr, Event, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
};

pub fn handle(deps: ProvDepsMut, _sender: Addr, contracts: Vec<Addr>) -> ProvTxResponse {
    let mut response = Response::default();
    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    for contract in &contracts {
        storage::contract::add(deps.storage, contract)?;
        response =
            response.add_event(Event::new("contract_added").add_attribute("address", contract));
    }
    Ok(response.add_attribute("action", "add_contracts"))
}
