use cosmwasm_std::{Addr, Env, Event, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
    util::is_contract_admin::is_contract_admin,
};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr, contracts: Vec<Addr>) -> ProvTxResponse {
    let mut response = Response::default();
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    for contract in &contracts {
        storage::contract::remove(deps.storage, contract);
        response =
            response.add_event(Event::new("contract_removed").add_attribute("address", contract));
    }
    Ok(response.add_attribute("action", "remove_contracts"))
}
