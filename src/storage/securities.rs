use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::Map;

use crate::core::{constants::SECURITIES_MAP_KEY, error::ContractError, security::Security};

// We store our securities that we configured on initialization
pub const SECURITIES_MAP: Map<String, Security> = Map::new(SECURITIES_MAP_KEY);

pub fn get_security_types(storage: &dyn Storage) -> Vec<String> {
    let security_types: StdResult<Vec<_>> = SECURITIES_MAP
        .keys(storage, None, None, Order::Ascending)
        .collect();
    security_types.unwrap()
}

pub fn get(storage: &dyn Storage, security_name: String) -> Result<Security, ContractError> {
    Ok(SECURITIES_MAP.load(storage, security_name)?)
}

pub fn set(storage: &mut dyn Storage, security: &Security) -> Result<(), ContractError> {
    Ok(SECURITIES_MAP.save(storage, security.name.clone(), security)?)
}
