use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

use crate::core::security::SecurityCommitment;

#[cw_serde]
pub struct Commitment {
    pub lp: Addr,
    pub commitments: Vec<SecurityCommitment>,
    pub state: CommitmentState,
    pub settlment_date: Option<Uint64>,
}

impl Commitment {
    pub fn new(lp: Addr, commitments: Vec<SecurityCommitment>) -> Self {
        Commitment {
            lp,
            commitments,
            state: CommitmentState::PENDING,
            settlment_date: None,
        }
    }

    pub fn clear_amounts(&mut self) {
        for commitment in &mut self.commitments {
            commitment.amount = Uint128::zero();
        }
    }
}

#[cw_serde]
pub enum CommitmentState {
    PENDING,
    ACCEPTED,
    SETTLED,
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint128};

    use crate::{
        core::security::SecurityCommitment, execute::settlement::commitment::CommitmentState,
    };

    use super::Commitment;

    #[test]
    fn test_clear_amounts() {
        let lp = Addr::unchecked("address");
        let securities = vec![SecurityCommitment {
            name: "security 1".to_string(),
            amount: Uint128::new(5),
        }];
        let mut commitment = Commitment::new(lp.clone(), securities.clone());

        commitment.clear_amounts();
        assert_eq!(securities.len(), commitment.commitments.len());
        for security in &commitment.commitments {
            assert_eq!(0, security.amount.u128());
        }
    }

    #[test]
    fn test_new() {
        let lp = Addr::unchecked("address");
        let securities = vec![SecurityCommitment {
            name: "security 1".to_string(),
            amount: Uint128::new(5),
        }];
        let commitment = Commitment::new(lp.clone(), securities.clone());

        assert_eq!(lp, commitment.lp);
        assert_eq!(CommitmentState::PENDING, commitment.state);
        assert_eq!(securities, commitment.commitments);
        assert_eq!(None, commitment.settlment_date);
    }
}
