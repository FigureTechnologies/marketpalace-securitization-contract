use cosmwasm_std::{Binary, Env};

use crate::{
    core::{
        aliases::{ProvDeps, ProvQueryResponse},
        msg::QueryMsg,
    },
    util::validate::{Validate, ValidateResult},
};

pub fn handle(_deps: ProvDeps, _env: Env, _msg: QueryMsg) -> ProvQueryResponse {
    Ok(Binary(Vec::new()))
}

impl Validate for QueryMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, _funds: &Vec<cosmwasm_std::Coin>) -> ValidateResult {
        Ok(())
    }
}
