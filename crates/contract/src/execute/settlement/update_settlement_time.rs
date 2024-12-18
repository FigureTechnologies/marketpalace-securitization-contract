use cosmwasm_std::{Addr, Response, Uint64};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::{commits, state},
};

pub fn handle(deps: ProvDepsMut, sender: Addr, settlement_time: Option<Uint64>) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        return Err(ContractError::Unauthorized {});
    }

    state::set_settlement_time(deps.storage, settlement_time)?;
    commits::set_settlement_time(deps.storage, settlement_time)?;
    Ok(Response::default().add_attribute("action", "update_settlement_time"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Uint64};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::error::ContractError,
        util::testing::{create_test_state, instantiate_contract},
    };

    #[test]
    fn test_handle_should_fail_if_sender_is_not_gp() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let sender = Addr::unchecked("lp");
        let settlement_time = Some(Uint64::new(9999));
        create_test_state(&mut deps, &env, false);
        let err = super::handle(deps.as_mut(), sender, settlement_time).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), err.to_string());
    }

    #[test]
    fn test_handle_should_succeed() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let sender = Addr::unchecked("gp");
        let settlement_time = Some(Uint64::new(9999));
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_test_state(&mut deps, &env, false);
        let res = super::handle(deps.as_mut(), sender, settlement_time).unwrap();
        assert_eq!(0, res.events.len());
        assert_eq!(
            vec![Attribute::new("action", "update_settlement_time")],
            res.attributes
        );
    }
}
