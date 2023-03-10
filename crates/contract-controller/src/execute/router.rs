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
    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        Attribute,
    };

    use crate::{
        execute,
        util::testing::{
            add_contracts, create_admin_deps, instantiate_contract, test_add_contracts_message,
            test_migrate_all_contracts_message, test_migrate_contracts_message,
            test_modify_batch_size_message, test_remove_contracts_message,
        },
    };

    #[test]
    fn test_execute_add_contracts_has_correct_response() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let message = test_add_contracts_message();
        let info = mock_info("admin", &[]);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = execute::router::route(deps.as_mut(), env, info, message).unwrap();

        assert_eq!(Attribute::new("action", "add_contracts"), res.attributes[0]);
    }

    #[test]
    fn test_execute_remove_contracts_has_correct_response() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let message = test_remove_contracts_message();
        let info = mock_info("admin", &[]);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts(deps.as_mut(), env.clone()).unwrap();
        let res = execute::router::route(deps.as_mut(), env, info, message).unwrap();

        assert_eq!(
            Attribute::new("action", "remove_contracts"),
            res.attributes[0]
        );
    }

    #[test]
    fn test_query_migrate_contracts_has_correct_response() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let message = test_migrate_contracts_message();
        let info = mock_info("admin", &[]);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts(deps.as_mut(), env.clone()).unwrap();
        let res = execute::router::route(deps.as_mut(), env, info, message).unwrap();

        assert_eq!(
            Attribute::new("action", "migrate_contracts"),
            res.attributes[0]
        );
    }

    #[test]
    fn test_query_migrate_all_contracts_has_correct_response() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let message = test_migrate_all_contracts_message();
        let info = mock_info("admin", &[]);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts(deps.as_mut(), env.clone()).unwrap();
        let res = execute::router::route(deps.as_mut(), env, info, message).unwrap();

        assert_eq!(
            Attribute::new("action", "migrate_all_contracts"),
            res.attributes[0]
        );
    }

    #[test]
    fn test_query_modify_batch_size_has_correct_response() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let message = test_modify_batch_size_message();
        let info = mock_info("admin", &[]);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = execute::router::route(deps.as_mut(), env, info, message).unwrap();

        assert_eq!(
            Attribute::new("action", "modify_batch_size"),
            res.attributes[0]
        );
    }
}
