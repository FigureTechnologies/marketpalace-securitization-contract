use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::ExecuteMsg,
};

use super::{
    add_contracts, migrate_all_contracts, migrate_contracts, modify_batch_size, remove_contracts,
};

pub fn route(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::AddContracts { contracts } => {
            add_contracts::handle(deps, env, info.sender, contracts)
        }
        ExecuteMsg::RemoveContracts { contracts } => {
            remove_contracts::handle(deps, env, info.sender, contracts)
        }
        ExecuteMsg::MigrateContracts {
            contracts,
            new_contract,
        } => migrate_contracts::handle(deps, env, info.sender, contracts, new_contract),
        ExecuteMsg::MigrateAllContracts { new_contract } => {
            migrate_all_contracts::handle(deps, env, info.sender, new_contract)
        }
        ExecuteMsg::ModifyBatchSize { batch_size } => {
            modify_batch_size::handle(deps, env, info.sender, batch_size.u128())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_execute_add_contracts_has_correct_response() {
        assert!(false);
    }

    #[test]
    fn test_execute_remove_contracts_has_correct_response() {
        assert!(false);
    }

    #[test]
    fn test_query_migrate_contracts_has_correct_response() {
        assert!(false);
    }

    #[test]
    fn test_query_migrate_all_contracts_has_correct_response() {
        assert!(false);
    }

    #[test]
    fn test_query_modify_batch_size_has_correct_response() {
        assert!(false);
    }
}
