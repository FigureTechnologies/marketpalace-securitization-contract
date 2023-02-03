use cosmwasm_std::Coin;

use crate::core::error::ContractError;

pub type ValidateResult = Result<(), ContractError>;
pub trait Validate {
    fn validate(&self) -> ValidateResult;
    fn validate_msg_funds(&self, funds: &Vec<Coin>) -> ValidateResult;
}
