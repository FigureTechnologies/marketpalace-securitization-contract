use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::{Bound, Map};

use crate::core::{constants::CONTRACT_KEY, error::ContractError};

// We store our securities that we configured on initialization
pub const CONTRACTS_MAP: Map<&Addr, bool> = Map::new(CONTRACT_KEY);

pub fn has(storage: &dyn Storage, contract: &Addr) -> bool {
    CONTRACTS_MAP.has(storage, contract)
}

pub fn add(storage: &mut dyn Storage, contract: &Addr) -> Result<(), ContractError> {
    Ok(CONTRACTS_MAP.save(storage, contract, &true)?)
}

pub fn remove(storage: &mut dyn Storage, contract: &Addr) {
    CONTRACTS_MAP.remove(storage, contract);
}

pub fn list(storage: &dyn Storage) -> Vec<Addr> {
    let contracts: Vec<Addr> = CONTRACTS_MAP
        .keys(storage, None, None, Order::Ascending)
        .map(|item| item.unwrap())
        .collect();
    contracts
}

pub fn range(storage: &mut dyn Storage, start: Option<&Addr>, amount: u128) -> Vec<Addr> {
    let min = start.map(Bound::exclusive);
    let contracts: Vec<Addr> = match amount {
        0 => CONTRACTS_MAP
            .keys(storage, min, None, Order::Ascending)
            .map(|item| item.unwrap())
            .collect(),
        _ => CONTRACTS_MAP
            .keys(storage, min, None, Order::Ascending)
            .take(amount as usize)
            .map(|item| item.unwrap())
            .collect(),
    };
    contracts
}
