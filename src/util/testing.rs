use cosmwasm_std::{Addr, Storage};

use crate::core::state::{State, STATE};

pub fn setup_tests() {
    // We want things added to STATE
    // We want things added to COMMITS
    // We want things added to PAID_IN_CAPITAL
    // We want a way for things added to
}

// We want a way to create a security commitment
// We want a way to create a commitment
// Maybe we want a way to easily transition between states for the settlement

pub fn setup_test_state(storage: &mut dyn Storage) {
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
