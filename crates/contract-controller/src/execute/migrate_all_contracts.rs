use cosmwasm_std::{Addr, Env, Response, Uint128};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
    util::{is_contract_admin::is_contract_admin, migrate_contracts::migrate_contracts},
};

// We may need to do batching on this because of the large amount of securities
pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr, contract_id: Uint128) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    let mut state = storage::state::get(deps.storage)?;
    state.migrating = true;

    let contracts =
        storage::contract::range(deps.storage, state.last_address.as_ref(), state.batch_size);
    let messages = migrate_contracts(deps.storage, &contracts, contract_id)?;

    // Automatically exit migrating
    if contracts.is_empty() {
        state.migrating = false;
    }
    state.last_address = contracts.last().cloned();
    storage::state::set(deps.storage, &state)?;
    Ok(Response::default()
        .add_attribute("migration_finished", contracts.is_empty().to_string())
        .add_attribute("action", "migrate_all_contracts")
        .add_submessages(messages))
}
