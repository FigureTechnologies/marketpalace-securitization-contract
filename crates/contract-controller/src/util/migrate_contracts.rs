use cosmwasm_std::{to_json_binary, Addr, Storage, SubMsg, Uint128, WasmMsg};

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
            msg: to_json_binary(&ContractMigrateMsg {})?,
        };
        let id = storage::reply::add(storage, contract)?;
        messages.push(SubMsg::reply_always(msg, id));
    }
    Ok(messages)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, to_json_binary, Addr, SubMsg, Uint128, WasmMsg};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::ContractMigrateMsg,
        util::{migrate_contracts::migrate_contracts, testing::instantiate_contract},
    };

    #[test]
    fn test_migrate_contracts_empty() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        instantiate_contract(deps.as_mut(), env).unwrap();
        let contracts: Vec<Addr> = vec![];
        let contract_id = Uint128::new(2);

        let migrate_results =
            migrate_contracts(deps.as_mut().storage, &contracts, contract_id).unwrap();

        assert_eq!(0, migrate_results.len());
    }

    #[test]
    fn test_migrate_contracts_non_empty() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        instantiate_contract(deps.as_mut(), env).unwrap();
        let contracts: Vec<Addr> = vec![Addr::unchecked("test_address")];
        let contract_id = Uint128::new(2);

        let migrate_results =
            migrate_contracts(deps.as_mut().storage, &contracts, contract_id).unwrap();

        let msg = WasmMsg::Migrate {
            contract_addr: "test_address".to_string(),
            new_code_id: 2,
            msg: to_json_binary(&ContractMigrateMsg {}).unwrap(),
        };
        let expected = SubMsg::reply_always(msg, 1);

        assert_eq!(1, migrate_results.len());
        assert_eq!(expected, migrate_results[0]);
    }
}
