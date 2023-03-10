use cosmwasm_std::{Env, Event, Reply, Response, SubMsgResult};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
};

pub fn handle(deps: ProvDepsMut, _env: Env, reply: Reply) -> ProvTxResponse {
    if !storage::reply::has(deps.storage, reply.id) {
        return Err(ContractError::UnrecognizedReplyId {});
    }
    let addr = storage::reply::remove(deps.storage, reply.id)?;
    let event = match reply.result {
        SubMsgResult::Ok(_response) => Event::new("migration").add_attributes(vec![
            ("contract", addr.to_string()),
            ("success", "true".to_string()),
            ("error", "".to_string()),
        ]),
        SubMsgResult::Err(error) => Event::new("migration").add_attributes(vec![
            ("contract", addr.to_string()),
            ("success", "false".to_string()),
            ("error", error),
        ]),
    };
    Ok(Response::default().add_event(event))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::mock_env, Addr, Attribute, Event, Reply, SubMsgResponse, SubMsgResult,
    };
    use provwasm_mocks::mock_dependencies;

    use crate::{core::error::ContractError, reply, storage, util::testing::instantiate_contract};

    #[test]
    fn test_invalid_reply() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let reply = Reply {
            id: 100,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: None,
            }),
        };
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = reply::handler::handle(deps.as_mut(), env, reply).unwrap_err();
        assert_eq!(
            ContractError::UnrecognizedReplyId {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_valid_success_reply() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let contract = Addr::unchecked("contract1");
        let reply = Reply {
            id: 0,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: None,
            }),
        };
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        storage::reply::add(deps.as_mut().storage, &contract).unwrap();
        let res = reply::handler::handle(deps.as_mut(), env, reply).unwrap();
        assert_eq!(
            vec![Event::new("migration").add_attributes(vec![
                Attribute::new("contract", contract.to_string()),
                Attribute::new("success", "true"),
                Attribute::new("error", "")
            ])],
            res.events
        );
    }

    #[test]
    fn test_valid_error_reply() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let contract = Addr::unchecked("contract1");
        let error = "error from contract";
        let reply = Reply {
            id: 0,
            result: SubMsgResult::Err(error.to_string()),
        };
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        storage::reply::add(deps.as_mut().storage, &contract).unwrap();
        let res = reply::handler::handle(deps.as_mut(), env, reply).unwrap();
        assert_eq!(
            vec![Event::new("migration").add_attributes(vec![
                Attribute::new("contract", contract.to_string()),
                Attribute::new("success", "false"),
                Attribute::new("error", error)
            ])],
            res.events
        );
    }
}
