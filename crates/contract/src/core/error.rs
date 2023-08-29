use cosmwasm_std::StdError;
use thiserror::Error;

use super::aliases::ProvTxResponse;

#[derive(Error, Debug, PartialEq)]
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

    #[error("The accepted commitment does not match the proposed commitment")]
    AcceptedAndProposalMismatch {},

    #[error("One or more commitments are in an invalid state")]
    InvalidCommitmentState {},

    #[error("Commitment has not been met")]
    CommitmentNotMet {},

    #[error("Settlment time for this commitment has expired")]
    SettlmentExpired {},

    #[error("This action cannot be performed on a settled commitment")]
    AlreadySettled {},

    #[error("This action cannot be performed on a accepted commitment")]
    AlreadyAccepted {},

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

    #[error("The deposit is empty")]
    EmptyDeposit {},

    #[error("The commitment will exceed the remaining amount of a security")]
    CommitmentExceedsRemainingSecurityAmount {},

    #[error("A commitment by this lp already exists")]
    CommitmentAlreadyExists {},

    #[error("A commitment by this lp has already been accepted")]
    CommitmentAlreadyAccepted {},

    #[error("Mismatch in the migrating contract name")]
    ContractNameMismatch {},

    #[error("Invalid migration version")]
    InvalidVersion {},

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Loan pool contributor not in whitelist")]
    NotInWhitelist {},

    #[error("Invalid marker: {message}")]
    InvalidMarker { message: String },

    #[error("Invalid address: {message}")]
    InvalidAddress { message: String },
}

pub fn contract_error(err: &str) -> ProvTxResponse {
    Err(ContractError::Std(StdError::generic_err(err)))
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
