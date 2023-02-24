use cosmwasm_std::Coin;

use crate::{
    core::msg::MigrateMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for MigrateMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
