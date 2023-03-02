use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::ExecuteMsg,
};

use super::{add_contracts, migrate_contracts, remove_contracts};

pub fn route(deps: ProvDepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::AddContracts { contracts } => {
            add_contracts::handle(deps, info.sender, contracts)
        }
        ExecuteMsg::RemoveContracts { contracts } => {
            remove_contracts::handle(deps, info.sender, contracts)
        }
        ExecuteMsg::MigrateContracts { new_contract } => {
            migrate_contracts::handle(deps, info.sender, new_contract)
        }
    }
}

#[cfg(test)]
mod tests {}
