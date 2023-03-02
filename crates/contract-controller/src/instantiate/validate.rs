use cosmwasm_std::Coin;

use crate::{
    core::{error::ContractError, msg::InstantiateMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, funds: &[Coin]) -> ValidateResult {
        if !funds.is_empty() {
            return Err(ContractError::UnexpectedFunds {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
