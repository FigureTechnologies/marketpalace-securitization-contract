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

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_dependencies;

    use crate::util::{
        testing::{create_test_query_contracts, create_test_query_verison},
        validate::Validate,
    };

    #[test]
    fn test_query_version_validate() {
        let deps = mock_dependencies(&[]);
        let message = create_test_query_verison();
        message.validate().unwrap();
        message.validate_msg_funds(&[]).unwrap();
    }

    #[test]
    fn test_query_state_validate() {
        let deps = mock_dependencies(&[]);
        let message = create_test_query_verison();
        message.validate().unwrap();
        message.validate_msg_funds(&[]).unwrap();
    }

    #[test]
    fn test_query_contracts_validate() {
        let deps = mock_dependencies(&[]);
        let message = create_test_query_contracts();
        message.validate().unwrap();
        message.validate_msg_funds(&[]).unwrap();
    }
}
