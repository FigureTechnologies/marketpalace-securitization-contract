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
mod tests {
    use cosmwasm_std::Coin;

    use crate::{
        core::error::ContractError,
        util::{testing::test_init_message, validate::Validate},
    };

    #[test]
    fn test_validate_always_succeeds() {
        let message = test_init_message();
        message.validate().unwrap();
    }

    #[test]
    fn test_funds_throw_error() {
        let message = test_init_message();
        let error = message
            .validate_msg_funds(&[Coin::new(50, "nhash")])
            .unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_no_funds_succeeds() {
        let message = test_init_message();
        message.validate_msg_funds(&[]).unwrap();
    }
}
