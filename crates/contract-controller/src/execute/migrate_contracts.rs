use cosmwasm_std::{to_binary, Addr, Env, Response, SubMsg, Uint128, WasmMsg};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvSubMsg, ProvTxResponse},
        error::ContractError,
        msg::ContractMigrateMsg,
    },
    storage,
    util::is_contract_admin::is_contract_admin,
};

// We may need to do batching on this because of the large amount of securities
pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    contracts: Vec<Addr>,
    contract_id: Uint128,
) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    let messages = migrate_contracts(&contracts, contract_id)?;

    Ok(Response::default()
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
        messages.push(SubMsg::reply_always(msg, 0));
    }
    Ok(messages)
}
