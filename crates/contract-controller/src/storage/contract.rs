use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::Map;

use crate::core::{constants::CONTRACT_KEY, error::ContractError};

// We store our securities that we configured on initialization
pub const CONTRACTS_MAP: Map<Addr, bool> = Map::new(CONTRACT_KEY);

pub fn has(storage: &dyn Storage, contract: Addr) -> bool {
    CONTRACTS_MAP.has(storage, contract)
}

pub fn add(storage: &mut dyn Storage, contract: Addr) -> Result<(), ContractError> {
    Ok(CONTRACTS_MAP.save(storage, contract, &true)?)
}

pub fn remove(storage: &mut dyn Storage, contract: Addr) {
    CONTRACTS_MAP.remove(storage, contract);
}

pub fn list(storage: &mut dyn Storage) -> Vec<Addr> {
    CONTRACTS_MAP
        .keys(storage, None, None, Order::Ascending)
        .map(|item| item.unwrap())
        .collect()
}
