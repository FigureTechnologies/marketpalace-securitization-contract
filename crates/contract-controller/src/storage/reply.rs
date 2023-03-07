use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::Map;

use crate::core::{constants::REPLIES_KEY, error::ContractError};

// We store our securities that we configured on initialization
pub const REPLIES_MAP: Map<u64, Addr> = Map::new(REPLIES_KEY);

pub fn add(storage: &mut dyn Storage, contract: &Addr) -> Result<u64, ContractError> {
    let index = get_next_index(storage)?;
    REPLIES_MAP.save(storage, index, contract)?;
    Ok(index)
}

pub fn remove(storage: &mut dyn Storage, index: u64) -> Result<Addr, ContractError> {
    let addr = REPLIES_MAP.load(storage, index)?;
    REPLIES_MAP.remove(storage, index);
    Ok(addr)
}

fn get_next_index(storage: &dyn Storage) -> Result<u64, ContractError> {
    let index = REPLIES_MAP
        .keys(storage, None, None, Order::Descending)
        .next();
    match index {
        None => Ok(0),
        Some(item) => Ok(item?),
    }
}
