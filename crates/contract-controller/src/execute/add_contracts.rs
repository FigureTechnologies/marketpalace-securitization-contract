use cosmwasm_std::{Addr, Env, Event, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        msg::Contract,
    },
    storage,
    util::is_contract_admin::is_contract_admin,
};

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    contracts: Vec<Contract>,
) -> ProvTxResponse {
    let mut response = Response::default();
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    for contract in &contracts {
        storage::uuid::add(deps.storage, &contract.uuid, &contract.address)?;
        storage::contract::add(deps.storage, &contract.address)?;
        response = response
            .add_event(Event::new("contract_added").add_attribute("address", &contract.address));
    }
    Ok(response.add_attribute("action", "add_contracts"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Event};

    use crate::{
        core::{error::ContractError, msg::Contract},
        execute::add_contracts::handle,
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
        let res = handle(deps.as_mut(), env, sender, contracts).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), res.to_string());
    }*/

    #[test]
    fn test_handle_is_not_in_migrating_state() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![
            Contract {
                address: Addr::unchecked("contract1"),
                uuid: "uuid1".to_string(),
            },
            Contract {
                address: Addr::unchecked("contract2"),
                uuid: "uuid2".to_string(),
            },
        ];

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let mut state = storage::state::get(&deps.storage).unwrap();
        state.migrating = true;
        storage::state::set(deps.as_mut().storage, &state).unwrap();

        let res = handle(deps.as_mut(), env, sender, contracts).unwrap_err();
        assert_eq!(
            ContractError::MigrationInProcess {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_handle_successfully_adds() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let contracts = vec![
            Contract {
                address: Addr::unchecked("contract1"),
                uuid: "uuid1".to_string(),
            },
            Contract {
                address: Addr::unchecked("contract2"),
                uuid: "uuid2".to_string(),
            },
        ];

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();

        let res = handle(deps.as_mut(), env, sender, contracts.clone()).unwrap();
        assert_eq!(
            vec![Attribute::new("action", "add_contracts")],
            res.attributes
        );
        assert_eq!(
            vec![
                Event::new("contract_added").add_attribute("address", "contract1"),
                Event::new("contract_added").add_attribute("address", "contract2")
            ],
            res.events
        );
        assert_eq!(
            contracts[0].address.clone(),
            storage::uuid::get(&deps.storage, contracts[0].uuid.as_str()).unwrap()
        );
        assert_eq!(
            contracts[1].address.clone(),
            storage::uuid::get(&deps.storage, contracts[1].uuid.as_str()).unwrap()
        );
        // This is where we want to check the store
    }
}
