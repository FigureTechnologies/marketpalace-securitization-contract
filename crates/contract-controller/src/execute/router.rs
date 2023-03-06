use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::ExecuteMsg,
};

use super::{add_contracts, migrate_contracts, modify_batch_size, remove_contracts};

pub fn route(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::AddContracts { contracts } => {
            add_contracts::handle(deps, env, info.sender, contracts)
        }
        ExecuteMsg::RemoveContracts { contracts } => {
            remove_contracts::handle(deps, env, info.sender, contracts)
        }
        ExecuteMsg::MigrateContracts { new_contract } => {
            migrate_contracts::handle(deps, env, info.sender, new_contract)
        }
        ExecuteMsg::ModifyBatchSize { batch_size } => {
            modify_batch_size::handle(deps, env, info.sender, batch_size.u128())
        }
    }
}

#[cfg(test)]
mod tests {}
