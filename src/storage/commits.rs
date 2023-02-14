use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Map;

use crate::{
    core::{constants::COMMITS_KEY, error::ContractError},
    execute::settlement::commitment::Commitment,
};

pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);

pub fn get(storage: &dyn Storage, lp: Addr) -> Result<Commitment, ContractError> {
    Ok(COMMITS.load(storage, lp)?)
}

pub fn set(storage: &mut dyn Storage, commitment: &Commitment) -> Result<(), ContractError> {
    Ok(COMMITS.save(storage, commitment.lp.clone(), commitment)?)
}
