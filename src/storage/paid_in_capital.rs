use cosmwasm_std::Addr;
use cw_storage_plus::Map;

use crate::core::{constants::PAID_IN_CAPITAL_KEY, security::SecurityCommitment};

pub const PAID_IN_CAPITAL: Map<Addr, Vec<SecurityCommitment>> = Map::new(PAID_IN_CAPITAL_KEY);
