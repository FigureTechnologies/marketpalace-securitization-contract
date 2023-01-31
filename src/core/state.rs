use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::commitment::Commitment;

use super::{
    constants::{
        AVAILABLE_CAPITAL_KEY, COMMITS_KEY, PAID_IN_CAPITAL_KEY, SECURITIES_MAP_KEY, STATE_KEY,
    },
    security::{Security, SecurityCommitment},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub gp: Addr,
    pub capital_denom: String,
}

impl State {
    pub fn new(gp: Addr, capital_denom: String) -> Self {
        Self { gp, capital_denom }
    }
}

pub const STATE: Item<State> = Item::new(STATE_KEY);
pub const SECURITIES_MAP: Map<String, Security> = Map::new(SECURITIES_MAP_KEY);
// TODO Have a single COMMIT map, but move the state into COMMIT
pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);
pub const PAID_IN_CAPITAL: Map<Addr, Vec<SecurityCommitment>> = Map::new(PAID_IN_CAPITAL_KEY);
pub const AVAILABLE_CAPITAL: Map<Addr, Vec<Coin>> = Map::new(AVAILABLE_CAPITAL_KEY);
