use cosmwasm_std::{to_binary, Addr, Response, SubMsg, Uint128, WasmMsg};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::MigrateMsg,
    },
    storage,
};

pub fn handle(deps: ProvDepsMut, _sender: Addr, contract_id: Uint128) -> ProvTxResponse {
    let mut response = Response::default();
    for contract in storage::contract::list(deps.storage) {
        let msg = WasmMsg::Migrate {
            contract_addr: contract.to_string(),
            new_code_id: contract_id.u128() as u64,
            msg: to_binary(&MigrateMsg {})?,
        };
        response = response.add_submessage(SubMsg::reply_on_success(msg, 0));
    }
    Ok(response)
}
