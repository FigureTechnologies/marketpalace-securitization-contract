use cosmwasm_std::StdError;
use thiserror::Error;

use super::aliases::ProvTxResponse;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Missing required funds")]
    MissingFunds {},

    #[error("Unexpected funds were added to this transaction")]
    UnexpectedFunds {},

    #[error("Duplicate contract supplied")]
    DuplicateContract {},

    #[error("The supplied contract list is empty")]
    EmptyContractList {},

    #[error("Unable to perform this action while migrating manages contracts")]
    MigrationInProcess {},

    #[error("One or more supplied contracts is not managed by this contract")]
    UnmanageContract {},

    #[error("This action can only be performed by the contract's admin")]
    Unauthorized {},

    #[error("Mismatch in the migrating contract name")]
    ContractNameMismatch {},

    #[error("Invalid migration version")]
    InvalidVersion {},

    #[error("Unrecognized reply id")]
    UnrecognizedReplyId {},

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Reply parsing error: {0}")]
    ParseReply(String),
}

pub fn contract_error(err: &str) -> ProvTxResponse {
    Err(ContractError::Std(StdError::generic_err(err)))
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

impl From<cw_utils::ParseReplyError> for ContractError {
    fn from(err: cw_utils::ParseReplyError) -> Self {
        Self::ParseReply(err.to_string())
    }
}
