use cosmwasm_std::{Addr, Storage};

use crate::{
    core::security::SecurityCommitment,
    storage::state::{State, STATE},
};

pub fn setup_tests() {
    // We want things added to STATE
    // We want things added to COMMITS
    // We want things added to PAID_IN_CAPITAL
    // We want a way for things added to
}

// We want a way to create a security commitment
// We want a way to create a commitment
// Maybe we want a way to easily transition between states for the settlement

pub struct SettlementTester {
    pub security_commitments: Vec<SecurityCommitment>,
}

impl SettlementTester {
    pub fn new() -> Self {
        SettlementTester {
            security_commitments: vec![],
        }
    }

    pub fn setup_test_state(&self, storage: &mut dyn Storage) {
        STATE
            .save(
                storage,
                &State {
                    gp: Addr::unchecked("gp"),
                    capital_denom: "denom".to_string(),
                    rules: vec![],
                },
            )
            .unwrap();
    }

    pub fn create_security_commitments(&mut self, amount: u32) {
        for _ in 0..amount {
            self.security_commitments.push(SecurityCommitment {
                name: format!("Security{}", self.security_commitments.len() + 1),
                amount: (self.security_commitments.len() + 11) as u128,
            });
        }
    }
}
