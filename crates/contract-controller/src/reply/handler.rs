use cosmwasm_std::{Env, Event, Reply, Response, SubMsgResult};

use crate::{
    core::aliases::{ProvDepsMut, ProvTxResponse},
    storage,
};

pub fn handle(deps: ProvDepsMut, _env: Env, reply: Reply) -> ProvTxResponse {
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
    #[test]
    fn test_invalid_reply() {
        assert!(false);
    }

    #[test]
    fn test_valid_reply() {
        assert!(false);
    }
}
