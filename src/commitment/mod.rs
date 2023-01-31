use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::core::msg::SecurityCommitment;

#[cw_serde]
pub struct Commitment {
    pub lp: Addr,
    pub commitments: Vec<SecurityCommitment>,
    pub state: CommitmentState,
}

impl Commitment {
    pub fn new(lp: Addr, commitments: Vec<SecurityCommitment>) -> Self {
        Commitment {
            lp,
            commitments,
            state: CommitmentState::PENDING,
        }
    }

    pub fn clear(&mut self) {
        for commitment in &mut self.commitments {
            commitment.amount = 0;
        }
    }
}

#[cw_serde]
pub enum CommitmentState {
    PENDING,
    ACCEPTED,
    SETTLED,
    INVESTED,
}
