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
mod tests {}
