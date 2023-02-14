use cosmwasm_std::Addr;
use cw_storage_plus::Map;

use crate::{core::constants::COMMITS_KEY, execute::settlement::commitment::Commitment};

pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);
