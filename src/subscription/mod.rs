use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::core::msg::SecurityCommitment;

#[cw_serde]
pub struct Subscription {
    pub lp: Addr,
    pub commitments: Vec<SecurityCommitment>,
}

impl Subscription {
    pub fn new(lp: Addr, commitments: Vec<SecurityCommitment>) -> Self {
        Subscription { lp, commitments }
    }
}
