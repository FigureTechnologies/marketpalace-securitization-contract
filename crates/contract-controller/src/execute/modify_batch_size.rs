use cosmwasm_std::{Addr, Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::state::update_batch_size,
    util::is_contract_admin::is_contract_admin,
};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr, batch_size: u128) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    update_batch_size(deps.storage, batch_size)?;
    Ok(Response::default()
        .add_attribute("action", "modify_batch_size")
        .add_attribute("new_batch_size", batch_size.to_string()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Event};

    use crate::{
        core::error::ContractError,
        execute::modify_batch_size,
        storage,
        util::testing::{create_admin_deps, instantiate_contract},
    };

    #[test]
    fn test_handle_sender_is_admin() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("sender");
        let res = modify_batch_size::handle(deps.as_mut(), env, sender, 5).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), res.to_string());
    }

    #[test]
    fn test_handle_modifies_batch_size() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let sender = Addr::unchecked("admin");

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();

        let res = modify_batch_size::handle(deps.as_mut(), env, sender, 5).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        assert_eq!(5, state.batch_size);
        assert_eq!(
            vec![
                Attribute::new("action", "modify_batch_size"),
                Attribute::new("new_batch_size", "5")
            ],
            res.attributes
        );
        assert_eq!(0, res.events.len());
    }
}
