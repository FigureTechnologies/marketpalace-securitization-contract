use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Addr, Coin, Storage,
};

use crate::{
    contract::instantiate,
    core::{
        aliases::{ProvDeps, ProvDepsMut, ProvTxResponse},
        msg::InstantiateMsg,
        security::{FundSecurity, Security, SecurityCommitment},
    },
    storage::state::{self, State},
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
        state::set(
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

impl Default for SettlementTester {
    fn default() -> Self {
        Self::new()
    }
}

pub fn test_init_message() -> InstantiateMsg {
    InstantiateMsg {
        gp: Addr::unchecked("gp"),
        securities: vec![
            Security {
                name: "Security1".to_string(),
                amount: 1000,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 100,
                price_per_unit: Coin::new(100, "denom".to_string()),
            },
            Security {
                name: "Security2".to_string(),
                amount: 1000,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 100,
                price_per_unit: Coin::new(100, "denom".to_string()),
            },
        ],
        capital_denom: "denom".to_string(),
        rules: vec![],
    }
}

pub fn instantiate_contract(deps: ProvDepsMut) -> ProvTxResponse {
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let msg = test_init_message();

    instantiate(deps, env, info, msg)
}
