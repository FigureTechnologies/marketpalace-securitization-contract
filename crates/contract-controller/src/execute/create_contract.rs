use cosmwasm_std::{to_binary, Addr, Env, Response, SubMsg, Uint64, WasmMsg};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        security,
    },
    storage,
    util::is_contract_admin::is_contract_admin,
};

// We may need to do batching on this because of the large amount of securities
pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    message: security::InstantiateMsg,
    code_id: Uint64,
) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    let msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: code_id.u64(),
        msg: to_binary(&message)?,
        funds: vec![],
        label: format!("securitization"),
    };
    let msg = SubMsg::reply_on_success(msg, 0);

    Ok(Response::default()
        .add_attribute("action", "create_contract")
        .add_submessage(msg))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Uint64};

    use crate::{
        core::error::ContractError,
        execute::create_contract,
        storage,
        util::testing::{
            create_admin_deps, instantiate_contract, instantiate_contract_message,
            test_create_contract_init_message,
        },
    };

    // TODO Add this back
    /*#[test]
    fn test_must_be_admin() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("sender");
        let message = test_create_contract_init_message();
        let contract_id = Uint64::new(2);

        let res =
            create_contract::handle(deps.as_mut(), env, sender, message, contract_id).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            res.to_string()
        );
    }*/

    #[test]
    fn test_is_not_in_migrating_state() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let message = test_create_contract_init_message();
        let contract_id = Uint64::new(2);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let mut state = storage::state::get(&deps.storage).unwrap();
        state.migrating = true;
        storage::state::set(deps.as_mut().storage, &state).unwrap();

        let res =
            create_contract::handle(deps.as_mut(), env, sender, message, contract_id).unwrap_err();
        assert_eq!(
            ContractError::MigrationInProcess {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_success_has_correct_response() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");
        let message = test_create_contract_init_message();
        let contract_id = Uint64::new(2);

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let mut state = storage::state::get(&deps.storage).unwrap();
        state.migrating = false;
        storage::state::set(deps.as_mut().storage, &state).unwrap();

        let res = create_contract::handle(deps.as_mut(), env.clone(), sender, message, contract_id)
            .unwrap();
        assert_eq!(0, res.events.len());
        assert_eq!(
            vec![Attribute::new("action", "create_contract")],
            res.attributes
        );
        assert_eq!(
            vec![instantiate_contract_message(
                env.contract.address,
                contract_id
            )],
            res.messages
        )
    }
}
