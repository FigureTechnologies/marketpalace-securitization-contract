use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Map;

use crate::core::constants::AVAILABLE_CAPITAL_KEY;

pub const AVAILABLE_CAPITAL: Map<Addr, Vec<Coin>> = Map::new(AVAILABLE_CAPITAL_KEY);
