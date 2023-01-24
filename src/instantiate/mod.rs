use cosmwasm_std::{Env, MessageInfo, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::InstantiateMsg,
    },
    util::validate::{Validate, ValidateResult},
};

pub fn run(
    _deps: ProvDepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ProvTxResponse {
    Ok(Response::new())
}

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
