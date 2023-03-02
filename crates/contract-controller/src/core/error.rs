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
