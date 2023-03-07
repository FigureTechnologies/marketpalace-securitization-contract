use cosmwasm_std::{to_binary, Addr, Storage, SubMsg, Uint128, WasmMsg};

use crate::{
    core::{aliases::ProvSubMsg, error::ContractError, msg::ContractMigrateMsg},
    storage,
};

pub fn migrate_contracts(
    storage: &mut dyn Storage,
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
        let id = storage::reply::add(storage, contract)?;
        messages.push(SubMsg::reply_always(msg, id));
    }
    Ok(messages)
}
