use cosmwasm_std::{Env, Response, Storage};
use semver::Version;

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::MigrateMsg,
    },
    util::validate::ValidateResult,
};

pub fn handle(deps: &ProvDepsMut, _env: Env, _msg: MigrateMsg) -> ProvTxResponse {
    validate_migration(deps.storage)?;

    // Do migration here

    Ok(Response::new())
}

fn validate_migration(storage: &dyn Storage) -> ValidateResult {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = cw2::get_contract_version(storage)?.version.parse().unwrap();
    let ver = cw2::get_contract_version(storage)?;

    if ver.contract != CONTRACT_NAME {
        return Err(crate::core::error::ContractError::ContractNameMismatch {});
    }

    if storage_version < version {
        return Err(crate::core::error::ContractError::InvalidVersion {});
    }

    Ok(())
}
