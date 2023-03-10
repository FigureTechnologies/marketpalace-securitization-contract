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
pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr, contract_id: Uint128) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    let mut state = storage::state::get(deps.storage)?;
    state.migrating = true;

    let contracts =
        storage::contract::range(deps.storage, state.last_address.as_ref(), state.batch_size);
    let messages = migrate_contracts(deps.storage, &contracts, contract_id)?;

    // Automatically exit migrating
    if contracts.is_empty() {
        state.migrating = false;
    }
    state.last_address = contracts.last().cloned();
    storage::state::set(deps.storage, &state)?;
    Ok(Response::default()
        .add_attribute("action", "migrate_all_contracts")
        .add_attribute("migration_finished", contracts.is_empty().to_string())
        .add_submessages(messages))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Uint128};

    use crate::{
        core::error::ContractError,
        execute::{add_contracts, migrate_all_contracts},
        storage,
        util::testing::{create_admin_deps, instantiate_contract, migrate_message},
    };

    #[test]
    fn test_handle_is_admin() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("sender");
        let contract_id = Uint128::new(2);
        let res =
            migrate_all_contracts::handle(deps.as_mut(), env, sender, contract_id).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), res.to_string());
    }

    #[test]
    fn test_handle_works_with_no_managed_contracts() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contract_id = Uint128::new(2);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = migrate_all_contracts::handle(deps.as_mut(), env, sender, contract_id).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();

        assert_eq!(false, state.migrating);
        assert_eq!(
            vec![
                Attribute::new("action", "migrate_all_contracts"),
                Attribute::new("migration_finished", "true")
            ],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_handle_migrates_in_batches() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contract_id = Uint128::new(2);
        let contracts = vec![
            Addr::unchecked("contract1"),
            Addr::unchecked("contract2"),
            Addr::unchecked("contract3"),
        ];

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();

        add_contracts::handle(
            deps.as_mut(),
            env.clone(),
            sender.clone(),
            contracts.clone(),
        )
        .unwrap();

        let res =
            migrate_all_contracts::handle(deps.as_mut(), env.clone(), sender.clone(), contract_id)
                .unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        assert_eq!(true, state.migrating);
        assert_eq!(
            vec![
                Attribute::new("action", "migrate_all_contracts"),
                Attribute::new("migration_finished", "false")
            ],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(
            vec![
                migrate_message(contracts[0].clone(), Uint128::new(2), 0),
                migrate_message(contracts[1].clone(), Uint128::new(2), 1)
            ],
            res.messages
        );

        let res =
            migrate_all_contracts::handle(deps.as_mut(), env.clone(), sender.clone(), contract_id)
                .unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        assert_eq!(true, state.migrating);
        assert_eq!(
            vec![
                Attribute::new("action", "migrate_all_contracts"),
                Attribute::new("migration_finished", "false")
            ],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(
            vec![migrate_message(contracts[2].clone(), Uint128::new(2), 2)],
            res.messages
        );

        let res =
            migrate_all_contracts::handle(deps.as_mut(), env.clone(), sender.clone(), contract_id)
                .unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        assert_eq!(false, state.migrating);
        assert_eq!(
            vec![
                Attribute::new("action", "migrate_all_contracts"),
                Attribute::new("migration_finished", "true")
            ],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.messages.len());
    }
}
