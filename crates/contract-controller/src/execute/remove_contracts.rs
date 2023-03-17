use cosmwasm_std::{Addr, Env, Event, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
    util::is_contract_admin::is_contract_admin,
};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr, contracts: Vec<Addr>) -> ProvTxResponse {
    let mut response = Response::default();
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    for contract in &contracts {
        if !storage::contract::has(deps.storage, contract) {
            return Err(ContractError::UnmanageContract {});
        }

        storage::contract::remove(deps.storage, contract);
        response =
            response.add_event(Event::new("contract_removed").add_attribute("address", contract));
    }
    Ok(response.add_attribute("action", "remove_contracts"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Event};

    use crate::{
        core::error::ContractError,
        execute::{
            add_contracts::{self},
            remove_contracts::{self},
        },
        storage,
        util::testing::{create_admin_deps, instantiate_contract},
    };

    // TODO Add this back
    /*#[test]
    fn test_handle_sender_is_admin() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("sender");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];
        let res = remove_contracts::handle(deps.as_mut(), env, sender, contracts).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), res.to_string());
    }*/

    #[test]
    fn test_handle_is_not_in_migrating_state() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let mut state = storage::state::get(&deps.storage).unwrap();
        state.migrating = true;
        storage::state::set(deps.as_mut().storage, &state).unwrap();

        let res = remove_contracts::handle(deps.as_mut(), env, sender, contracts).unwrap_err();
        assert_eq!(
            ContractError::MigrationInProcess {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_handle_cannot_remove_unmanaged() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();

        let res = remove_contracts::handle(deps.as_mut(), env, sender, contracts).unwrap_err();
        assert_eq!(
            ContractError::UnmanageContract {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_handle_successfully_removes() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![Addr::unchecked("contract1"), Addr::unchecked("contract2")];

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts::handle(
            deps.as_mut(),
            env.clone(),
            sender.clone(),
            contracts.clone(),
        )
        .unwrap();
        let res = remove_contracts::handle(deps.as_mut(), env, sender, contracts).unwrap();
        assert_eq!(
            vec![Attribute::new("action", "remove_contracts")],
            res.attributes
        );
        assert_eq!(
            vec![
                Event::new("contract_removed").add_attribute("address", "contract1"),
                Event::new("contract_removed").add_attribute("address", "contract2")
            ],
            res.events
        );
    }
}
