use crate::core::error::ContractError;

pub type ValidateResult = Result<(), ContractError>;
pub trait Validate {
    fn validate(&self) -> ValidateResult;
    fn requires_funds(&self) -> bool;
}
