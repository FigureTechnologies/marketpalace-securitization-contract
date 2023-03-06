use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{constants::STATE_KEY, error::ContractError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub batch_size: u128,
    pub migrating: bool,
    pub last_address: Option<Addr>,
}

impl State {
    pub fn new(batch_size: u128) -> Self {
        Self {
            batch_size,
            migrating: false,
            last_address: None,
        }
    }
}

// We store basic contract State
pub const STATE: Item<State> = Item::new(STATE_KEY);

pub fn get(storage: &dyn Storage) -> Result<State, ContractError> {
    Ok(STATE.load(storage)?)
}

pub fn set(storage: &mut dyn Storage, state: &State) -> Result<(), ContractError> {
    Ok(STATE.save(storage, state)?)
}

pub fn update_batch_size(
    storage: &mut dyn Storage,
    batch_size: u128,
) -> Result<State, ContractError> {
    STATE.update(storage, |mut state| -> Result<State, ContractError> {
        state.batch_size = batch_size;
        Ok(state)
    })
}

pub fn is_migrating(storage: &dyn Storage) -> Result<bool, ContractError> {
    let state = get(storage)?;
    Ok(state.migrating)
}

#[cfg(test)]
mod tests {}
