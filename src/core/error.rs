use cosmwasm_std::StdError;
use thiserror::Error;

use super::aliases::ProvTxResponse;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid security list")]
    InvalidSecurityList {},

    #[error("Empty security list")]
    EmptySecurityList {},

    #[error("Empty security list")]
    EmptySecurityCommitmentList {},

    #[error("Invalid security commitment")]
    InvalidSecurityCommitment {},

    #[error("Invalid security commitment amount")]
    InvalidSecurityCommitmentAmount {},

    #[error("Empty accepted commitment list")]
    EmptyAcceptedCommitmentList {},

    #[error("One or more commitments are in an invalid state")]
    InvalidCommitmentState {},

    #[error("Missing required funds")]
    MissingFunds {},

    #[error("Mismatch in the expected number of funds and the actual sent funds")]
    FundMismatch {},

    #[error("Unexpected funds were added to this transaction")]
    UnexpectedFunds {},

    #[error("The capital denom is invalid")]
    InvalidCapitalDenom {},

    #[error("The deposit exceeds the commitment amount")]
    ExcessiveDeposit {},
}

pub fn contract_error(err: &str) -> ProvTxResponse {
    Err(ContractError::Std(StdError::generic_err(err)))
}
