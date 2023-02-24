use cosmwasm_std::{
    testing::{mock_env, mock_info, MockApi, MockStorage},
    Addr, Coin, Env, OwnedDeps, Storage, Uint128,
};
use provwasm_mocks::ProvenanceMockQuerier;
use provwasm_std::ProvenanceQuery;

use crate::{
    contract::{execute, instantiate},
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::{ExecuteMsg, InstantiateMsg},
        security::{FundSecurity, Security, SecurityCommitment},
    },
    storage::state::{self, State},
};

#[cfg(tests)]

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
                amount: Uint128::new((self.security_commitments.len() + 11) as u128),
            });
        }
    }
}

impl Default for SettlementTester {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_test_securities() -> Vec<Security> {
    vec![
        Security {
            name: "Security1".to_string(),
            amount: Uint128::new(1000),
            security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
            minimum_amount: Uint128::new(10),
            price_per_unit: Coin::new(100, "denom".to_string()),
        },
        Security {
            name: "Security2".to_string(),
            amount: Uint128::new(1000),
            security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
            minimum_amount: Uint128::new(10),
            price_per_unit: Coin::new(100, "denom".to_string()),
        },
    ]
}

pub fn test_init_message() -> InstantiateMsg {
    InstantiateMsg {
        gp: Addr::unchecked("gp"),
        securities: create_test_securities(),
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

pub fn test_security_commitments() -> Vec<SecurityCommitment> {
    vec![
        SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(100),
        },
        SecurityCommitment {
            name: "Security2".to_string(),
            amount: Uint128::new(100),
        },
    ]
}

pub fn test_propose_message() -> ExecuteMsg {
    ExecuteMsg::ProposeCommitment {
        securities: test_security_commitments(),
    }
}

pub fn propose_test_commitment(deps: ProvDepsMut, env: Env, sender: &str) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_propose_message();
    execute(deps, env, info, msg)
}

pub fn test_accept_message(lps: &[&str]) -> ExecuteMsg {
    ExecuteMsg::AcceptCommitment {
        commitments: lps.iter().map(|lp| Addr::unchecked(lp.clone())).collect(),
    }
}

pub fn accept_test_commitment(
    deps: ProvDepsMut,
    env: Env,
    sender: &str,
    lps: &[&str],
) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_accept_message(lps);
    execute(deps, env, info, msg)
}

pub fn test_deposit_message(deposit: &[SecurityCommitment]) -> ExecuteMsg {
    ExecuteMsg::DepositCommitment {
        securities: deposit.to_vec(),
    }
}

pub fn deposit_test(
    deps: ProvDepsMut,
    env: Env,
    sender: &str,
    deposit: &[SecurityCommitment],
) -> ProvTxResponse {
    let funds = deposit
        .iter()
        .fold(Coin::new(0, "denom".to_string()), |acc, commit| -> Coin {
            Coin::new((commit.amount.u128() * 100) + acc.amount.u128(), acc.denom)
        });
    let info = mock_info(sender, &vec![funds]);
    let msg = test_deposit_message(deposit);
    execute(deps, env, info, msg)
}

pub fn withdraw_test(deps: ProvDepsMut, env: Env, sender: &str) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_withdraw_message();
    execute(deps, env, info, msg)
}

pub fn test_withdraw_message() -> ExecuteMsg {
    ExecuteMsg::WithdrawCommitments {}
}

pub type MockDeps = OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery>;

pub fn create_testing_commitments(deps: &mut MockDeps) {
    // Multiple LPs propose
    propose_test_commitment(deps.as_mut(), mock_env(), "lp1").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp2").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp3").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp4").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp5").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp6").expect("should be able to propose");

    // Accept 1,2,3
    accept_test_commitment(deps.as_mut(), mock_env(), "gp", &vec!["lp1", "lp2", "lp3"])
        .expect("should be able to accept defined lps");

    // Deposit 1 completely, and partial 2
    deposit_test(
        deps.as_mut(),
        mock_env(),
        "lp1",
        &test_security_commitments(),
    )
    .expect("should be able to deposit full amount");

    let mut partial_commitments = test_security_commitments();
    for partial_commitment in &mut partial_commitments {
        partial_commitment.amount = partial_commitment.amount / Uint128::new(2);
    }

    deposit_test(deps.as_mut(), mock_env(), "lp2", &partial_commitments)
        .expect("should be able to deposit partial amount");
}
