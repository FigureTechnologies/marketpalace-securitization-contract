use cosmwasm_std::{Coin, Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::MigrateMsg,
    },
    util::validate::{Validate, ValidateResult},
};

pub fn handle(_deps: ProvDepsMut, _env: Env, _msg: MigrateMsg) -> ProvTxResponse {
    Ok(Response::new())
}

impl Validate for MigrateMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}