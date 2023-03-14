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

    #[error("Commitment has not been met")]
    CommitmentNotMet {},

    #[error("Missing required funds")]
    MissingFunds {},

    #[error("Mismatch in the expected number of funds and the actual sent funds")]
    FundMismatch {},

    #[error("Unexpected funds were added to this transaction")]
    UnexpectedFunds {},

    #[error("The capital denom is invalid")]
    InvalidCapitalDenom {},

    #[error("All security denoms must match the capital denom")]
    InvalidSecurityPriceDenom {},

    #[error("The deposit exceeds the commitment amount")]
    ExcessiveDeposit {},

    #[error("The commitment will exceed the remaining amount of a security")]
    CommitmentExceedsRemainingSecurityAmount {},

    #[error("A commitment by this lp already exists")]
    CommitmentAlreadyExists {},

    #[error("Mismatch in the migrating contract name")]
    ContractNameMismatch {},

    #[error("Invalid migration version")]
    InvalidVersion {},

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

pub fn contract_error(err: &str) -> ProvTxResponse {
    Err(ContractError::Std(StdError::generic_err(err)))
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
