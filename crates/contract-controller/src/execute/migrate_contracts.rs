use cosmwasm_std::{to_binary, Addr, Response, SubMsg, Uint128, WasmMsg};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvSubMsg, ProvTxResponse},
        error::ContractError,
        msg::ContractMigrateMsg,
    },
    storage,
};

// We may need to do batching on this because of the large amount of securities
pub fn handle(deps: ProvDepsMut, _sender: Addr, contract_id: Uint128) -> ProvTxResponse {
    let mut state = storage::state::get(deps.storage)?;
    state.migrating = true;

    let contracts =
        storage::contract::range(deps.storage, state.last_address.as_ref(), state.batch_size);
    let messages = migrate_contracts(&contracts, contract_id)?;

    // Automatically exit migrating
    if contracts.is_empty() {
        state.migrating = false;
    }
    state.last_address = contracts.last().cloned();
    storage::state::set(deps.storage, &state)?;
    Ok(Response::default()
        .add_attribute("migration_finished", contracts.is_empty().to_string())
        .add_attribute("action", "migrate_contracts")
        .add_submessages(messages))
}

fn migrate_contracts(
    contracts: &Vec<Addr>,
    contract_id: Uint128,
) -> Result<Vec<ProvSubMsg>, ContractError> {
    let mut messages = vec![];
    for contract in contracts {
        let msg = WasmMsg::Migrate {
            contract_addr: contract.to_string(),
            new_code_id: contract_id.u128() as u64,
            msg: to_binary(&ContractMigrateMsg {})?,
        };
        messages.push(SubMsg::reply_on_success(msg, 0));
    }
    Ok(messages)
}
