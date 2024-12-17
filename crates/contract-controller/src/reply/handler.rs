use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants,
        error::ContractError,
    },
    storage,
};
use cosmwasm_std::{Env, Event, Reply, Response, SubMsgResult};
use provwasm_std::types::cosmwasm::wasm::v1beta1::MsgInstantiateContractResponse;

pub fn handle(deps: ProvDepsMut, env: Env, reply: Reply) -> ProvTxResponse {
    if reply.id == constants::REPLY_INIT_ID {
        on_init_reply(deps, env, reply)
    } else {
        on_migrate_reply(deps, env, reply)
    }
}

pub fn on_init_reply(deps: ProvDepsMut, _env: Env, reply: Reply) -> ProvTxResponse {
    let data = match &reply.result {
        SubMsgResult::Ok(response) => response.data.as_ref(),
        SubMsgResult::Err(_) => None,
    }
    .ok_or_else(|| {
        ContractError::ParseReply("Invalid reply from sub-message: Missing reply data".to_string())
    })?;

    let response = cw_utils::parse_instantiate_response_data(data)?;
    let contract_address = deps.api.addr_validate(&response.contract_address)?;
    storage::contract::add(deps.storage, &contract_address)?;
    let uuid = storage::uuid::get_last_uuid(deps.storage)?;
    storage::uuid::add(deps.storage, uuid.as_str(), &contract_address)?;
    Ok(Response::default())
}

pub fn on_migrate_reply(deps: ProvDepsMut, _env: Env, reply: Reply) -> ProvTxResponse {
    if !storage::reply::has(deps.storage, reply.id) {
        return Err(ContractError::UnrecognizedReplyId {});
    }
    let addr = storage::reply::remove(deps.storage, reply.id)?;
    let event = match reply.result {
        SubMsgResult::Ok(_response) => Event::new("migration").add_attributes(vec![
            ("contract", addr.to_string()),
            ("success", "true".to_string()),
            ("error", "none".to_string()),
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
    use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
    use cosmwasm_std::{
        testing::mock_env, Addr, Attribute, Binary, Event, Reply, SubMsgResponse, SubMsgResult,
    };
    use prost::Message;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{core::error::ContractError, reply, storage, util::testing::instantiate_contract};

    #[derive(Clone, PartialEq, Message)]
    struct MsgInstantiateContractResponse {
        #[prost(string, tag = "1")]
        pub contract_address: ::prost::alloc::string::String,
        #[prost(bytes, tag = "2")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }

    #[test]
    fn test_invalid_init_reply() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let reply = Reply {
            id: 0,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: None,
                msg_responses: vec![],
            }),
            gas_used: 50,
            payload: Binary::from("dummyData".as_bytes()),
        };
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let error = reply::handler::handle(deps.as_mut(), env, reply).unwrap_err();
        assert_eq!(
            ContractError::ParseReply(
                "Invalid reply from sub-message: Missing reply data".to_string()
            )
            .to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_valid_init_reply() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let uuid = "uuid";

        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: deps.api.addr_make(MOCK_CONTRACT_ADDR).to_string(),
            data: vec![],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());
        // The data must encode successfully
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        // Build reply message
        let reply = Reply {
            id: 0,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
                msg_responses: vec![],
            }),
            gas_used: 50,
            payload: Binary::from("dummyData".as_bytes()),
        };

        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        storage::uuid::set_last_uuid(deps.as_mut().storage, &uuid.to_string()).unwrap();
        let res = reply::handler::handle(deps.as_mut(), env, reply).unwrap();
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.attributes.len());
        assert_eq!(
            true,
            storage::contract::has(&deps.storage, &deps.api.addr_make(MOCK_CONTRACT_ADDR))
        );
        assert_eq!(true, storage::uuid::has(&deps.storage, uuid));
    }

    #[test]
    fn test_invalid_reply() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let reply = Reply {
            id: 100,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: None,
                msg_responses: vec![],
            }),
            gas_used: 50,
            payload: Binary::from("dummyData".as_bytes()),
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
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let contract = Addr::unchecked("contract1");
        let reply = Reply {
            id: 1,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: None,
                msg_responses: vec![],
            }),
            gas_used: 50,
            payload: Binary::from("dummyData".as_bytes()),
        };
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        storage::reply::add(deps.as_mut().storage, &contract).unwrap();
        let res = reply::handler::handle(deps.as_mut(), env, reply).unwrap();
        assert_eq!(
            vec![Event::new("migration").add_attributes(vec![
                Attribute::new("contract", contract.to_string()),
                Attribute::new("success", "true"),
                Attribute::new("error", "none")
            ])],
            res.events
        );
    }

    #[test]
    fn test_valid_error_reply() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let contract = Addr::unchecked("contract1");
        let error = "error from contract";
        let reply = Reply {
            id: 1,
            result: SubMsgResult::Err(error.to_string()),
            gas_used: 50,
            payload: Binary::from("dummyData".as_bytes()),
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
