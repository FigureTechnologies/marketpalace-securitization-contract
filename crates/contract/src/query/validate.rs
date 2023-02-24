use cosmwasm_std::Coin;

use crate::{
    core::msg::QueryMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for QueryMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
