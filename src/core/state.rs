use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::commitment::Commitment;

use super::{
    constants::{
        ACCEPTED_COMMITS_KEY, COMMITS_KEY, PENDING_COMMITS_KEY, SECURITIES_LIST_KEY,
        SECURITIES_MAP_KEY, STATE_KEY,
    },
    msg::Security,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub gp: Addr,
}

impl State {
    pub fn new(gp: Addr) -> Self {
        Self { gp }
    }
}

pub const STATE: Item<State> = Item::new(STATE_KEY);
pub const SECURITIES_MAP: Map<String, Security> = Map::new(SECURITIES_MAP_KEY);
pub const SECURITIES_LIST: Item<Vec<Security>> = Item::new(SECURITIES_LIST_KEY);
// TODO Have a single COMMIT map, but move the state into COMMIT
pub const PENDING: Map<Addr, Commitment> = Map::new(PENDING_COMMITS_KEY);
pub const ACCEPTED: Map<Addr, Commitment> = Map::new(ACCEPTED_COMMITS_KEY);
pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);
