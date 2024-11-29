use cosmwasm_std::{Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::InstantiateMsg,
    },
    storage::{self, state::State},
};

pub fn handle(
    deps: ProvDepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::new(msg.batch_size.u128());
    storage::state::set(deps.storage, &state)?;
    Ok(Response::default().add_attribute("action", "init"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_env, message_info},
        Attribute,
    };
    use cw2::get_contract_version;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::constants::{CONTRACT_NAME, CONTRACT_VERSION},
        storage::{self, state::State},
        util::testing::test_init_message,
    };

    use super::handle;

    #[test]
    fn test_proper_instantiation() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = test_init_message();

        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        let version = get_contract_version(&deps.storage).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();

        assert_eq!(CONTRACT_NAME, version.contract);
        assert_eq!(CONTRACT_VERSION, version.version);
        assert_eq!(State::new(2), state);
        assert_eq!(vec![Attribute::new("action", "init")], res.attributes);
    }
}
