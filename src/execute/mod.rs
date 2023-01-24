use cosmwasm_std::{Env, MessageInfo, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
    },
    util::validate::{Validate, ValidateResult},
};

pub fn run(_deps: ProvDepsMut, _env: Env, _info: MessageInfo, _msg: ExecuteMsg) -> ProvTxResponse {
    Ok(Response::new())
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
