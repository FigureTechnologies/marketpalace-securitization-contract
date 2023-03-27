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

    let all_managed = contracts
        .iter()
        .all(|contract| storage::contract::has(deps.storage, contract));
    if !all_managed {
        return Err(ContractError::UnmanageContract {});
    }

    let messages = migrate_contracts(deps.storage, &contracts, contract_id)?;

    Ok(Response::default()
        .add_attribute("action", "migrate_contracts")
        .add_submessages(messages))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Uint128};

    use crate::{
        core::error::ContractError,
        execute::{add_contracts, migrate_contracts},
        storage,
        util::testing::{create_admin_deps, instantiate_contract, migrate_message},
    };

    // TODO Add this back
    /*#[test]
    fn test_must_be_admin() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("sender");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];
        let contract_id = Uint128::new(2);

        let res = migrate_contracts::handle(deps.as_mut(), env, sender, contracts, contract_id)
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), res.to_string());
    }*/

    #[test]
    fn test_is_not_in_migrating_state() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];
        let contract_id = Uint128::new(2);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let mut state = storage::state::get(&deps.storage).unwrap();
        state.migrating = true;
        storage::state::set(deps.as_mut().storage, &state).unwrap();

        let res = migrate_contracts::handle(deps.as_mut(), env, sender, contracts, contract_id)
            .unwrap_err();
        assert_eq!(
            ContractError::MigrationInProcess {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_migrate_contracts_must_all_exist() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];
        let contract_id = Uint128::new(2);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();

        let res =
            migrate_contracts::handle(deps.as_mut(), env, sender, contracts.clone(), contract_id)
                .unwrap_err();
        assert_eq!(
            ContractError::UnmanageContract {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_migrate_contracts_works() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];
        let contract_id = Uint128::new(2);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts::handle(
            deps.as_mut(),
            env.clone(),
            sender.clone(),
            contracts.clone(),
        )
        .unwrap();

        let res =
            migrate_contracts::handle(deps.as_mut(), env, sender, contracts.clone(), contract_id)
                .unwrap();
        assert_eq!(
            vec![Attribute::new("action", "migrate_contracts")],
            res.attributes
        );
        assert_eq!(
            vec![
                migrate_message(contracts[0].clone(), Uint128::new(2), 1),
                migrate_message(contracts[1].clone(), Uint128::new(2), 2)
            ],
            res.messages
        )
    }
}
