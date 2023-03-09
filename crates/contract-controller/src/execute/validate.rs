use cosmwasm_std::Coin;

use crate::{
    core::{error::ContractError, msg::ExecuteMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        match self {
            ExecuteMsg::AddContracts { contracts } => {
                if contracts.is_empty() {
                    return Err(ContractError::EmptyContractList {});
                }
            }
            ExecuteMsg::RemoveContracts { contracts } => {
                if contracts.is_empty() {
                    return Err(ContractError::EmptyContractList {});
                }
            }
            ExecuteMsg::MigrateContracts {
                contracts,
                new_contract: _,
            } => {
                if contracts.is_empty() {
                    return Err(ContractError::EmptyContractList {});
                }
            }
            _ => {}
        };
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
        util::{
            testing::{
                test_add_contracts_empty_message, test_add_contracts_message,
                test_migrate_all_contracts_message, test_migrate_contracts_empty_message,
                test_migrate_contracts_message, test_modify_batch_size_message,
                test_remove_contracts_empty_message, test_remove_contracts_message,
            },
            validate::Validate,
        },
    };

    #[test]
    fn test_add_contracts_should_not_be_empty() {
        let message = test_add_contracts_empty_message();
        let error = message.validate().unwrap_err();
        assert_eq!(
            ContractError::EmptyContractList {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_add_contracts_should_pass_with_addresses() {
        let message = test_add_contracts_message();
        message.validate().unwrap();
    }

    #[test]
    fn test_add_contracts_should_not_have_funds() {
        let message = test_add_contracts_message();
        let error = message
            .validate_msg_funds(&[Coin::new(5, "nhash")])
            .unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_add_contracts_should_pass_without_funds() {
        let message = test_add_contracts_message();
        message.validate_msg_funds(&[]).unwrap();
    }

    #[test]
    fn test_remove_contracts_should_not_be_empty() {
        let message = test_remove_contracts_empty_message();
        let error = message.validate().unwrap_err();
        assert_eq!(
            ContractError::EmptyContractList {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_remove_contracts_should_pass_with_addresses() {
        let message = test_remove_contracts_message();
        message.validate().unwrap();
    }

    #[test]
    fn test_remove_contracts_should_not_have_funds() {
        let message = test_remove_contracts_message();
        let error = message
            .validate_msg_funds(&[Coin::new(5, "nhash")])
            .unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_remove_contracts_should_pass_without_funds() {
        let message = test_remove_contracts_message();
        message.validate_msg_funds(&[]).unwrap();
    }

    #[test]
    fn test_migrate_contracts_should_not_be_empty() {
        let message = test_migrate_contracts_empty_message();
        let error = message.validate().unwrap_err();
        assert_eq!(
            ContractError::EmptyContractList {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_migrate_contracts_should_pass_with_addresses() {
        let message = test_migrate_contracts_message();
        message.validate().unwrap();
    }

    #[test]
    fn test_migrate_contracts_should_not_have_funds() {
        let message = test_migrate_contracts_message();
        let error = message
            .validate_msg_funds(&[Coin::new(5, "nhash")])
            .unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_migrate_contracts_should_pass_without_funds() {
        let message = test_migrate_contracts_message();
        message.validate_msg_funds(&[]).unwrap();
    }

    #[test]
    fn test_migrate_all_contracts_should_pass() {
        let message = test_migrate_all_contracts_message();
        message.validate().unwrap();
    }

    #[test]
    fn test_migrate_all_contracts_should_not_have_funds() {
        let message = test_migrate_all_contracts_message();
        let error = message
            .validate_msg_funds(&[Coin::new(5, "nhash")])
            .unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_migrate_all_contracts_should_pass_without_funds() {
        let message = test_migrate_all_contracts_message();
        message.validate_msg_funds(&[]).unwrap();
    }

    #[test]
    fn test_modify_batch_size_should_pass() {
        let message = test_modify_batch_size_message();
        message.validate().unwrap();
    }

    #[test]
    fn test_modify_batch_size_should_not_have_funds() {
        let message = test_modify_batch_size_message();
        let error = message
            .validate_msg_funds(&[Coin::new(5, "nhash")])
            .unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_modify_batch_size_should_pass_without_funds() {
        let message = test_modify_batch_size_message();
        message.validate_msg_funds(&[]).unwrap();
    }
}
