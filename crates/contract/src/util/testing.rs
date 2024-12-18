use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{
    testing::{message_info, mock_env, MockApi, MockStorage},
    Addr, Coin, Env, OwnedDeps, Storage, Uint128, Uint64,
};
use provwasm_mocks::MockProvenanceQuerier;

use crate::{
    contract::{execute, instantiate},
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::{ExecuteMsg, InstantiateMsg},
        security::{AcceptedCommitment, FundSecurity, Security, SecurityCommitment},
    },
    storage::{
        self,
        state::{self, State},
    },
};

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
                settlement_time: None,
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
            price_per_unit: Coin::new(Uint128::new(100), "denom".to_string()),
        },
        Security {
            name: "Security2".to_string(),
            amount: Uint128::new(1000),
            security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
            minimum_amount: Uint128::new(10),
            price_per_unit: Coin::new(Uint128::new(100), "denom".to_string()),
        },
    ]
}

pub fn test_init_message() -> InstantiateMsg {
    InstantiateMsg {
        gp: Addr::unchecked("gp"),
        securities: create_test_securities(),
        capital_denom: "denom".to_string(),
        settlement_time: None,
        fee: None,
    }
}

pub fn instantiate_contract(deps: ProvDepsMut) -> ProvTxResponse {
    let env = mock_env();
    let info = message_info(&Addr::unchecked("sender"), &[]);
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

pub fn test_create_accepted_commitments(lps: &[&str]) -> Vec<AcceptedCommitment> {
    lps.iter()
        .map(|lp| AcceptedCommitment {
            lp: Addr::unchecked(lp.to_owned()),
            securities: test_security_commitments(),
        })
        .collect()
}

pub fn test_accept_message(lps: &[&str]) -> ExecuteMsg {
    ExecuteMsg::AcceptCommitment {
        commitments: test_create_accepted_commitments(lps),
    }
}

pub fn test_cancel_message(lp: &str) -> ExecuteMsg {
    ExecuteMsg::CancelCommitment {
        lp: Addr::unchecked(lp),
    }
}

pub fn cancel_test(deps: ProvDepsMut, env: Env, sender: &str, lp: &str) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_cancel_message(lp);
    execute(deps, env, info, msg)
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
    let funds = Vec::new();
    let info = mock_info(sender, &funds);
    let msg = test_deposit_message(deposit);
    execute(deps, env, info, msg)
}

pub fn withdraw_test(deps: ProvDepsMut, env: Env, sender: &str, lp: &str) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_withdraw_message(lp);
    execute(deps, env, info, msg)
}

pub fn test_withdraw_message(lp: &str) -> ExecuteMsg {
    ExecuteMsg::WithdrawCommitment {
        lp: Addr::unchecked(lp),
    }
}

pub fn withdraw_all_commitments_test(deps: ProvDepsMut, env: Env, sender: &str) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_withdraw_all_commitments_message();
    execute(deps, env, info, msg)
}

pub fn test_withdraw_all_commitments_message() -> ExecuteMsg {
    ExecuteMsg::WithdrawAllCommitments {}
}

pub type MockDeps = OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier>;

pub fn update_settlement_time_test(deps: ProvDepsMut, env: Env, sender: &str) -> ProvTxResponse {
    let info = mock_info(sender, &[]);
    let msg = test_update_settlement_time_message();
    execute(deps, env, info, msg)
}

pub fn test_update_settlement_time_message() -> ExecuteMsg {
    ExecuteMsg::UpdateSettlementTime {
        settlement_time: Some(Uint64::new(99999)),
    }
}

pub fn create_test_state(deps: &mut MockDeps, env: &Env, has_settlement: bool) {
    let settlement_time = match has_settlement {
        true => Some(Uint64::new(86400) + Uint64::new(env.block.time.seconds())),
        false => None,
    };
    let state = State::new(Addr::unchecked("gp"), "denom".to_string(), settlement_time);
    storage::state::set(deps.as_mut().storage, &state).unwrap();
}

pub fn create_testing_commitments(deps: &mut MockDeps) {
    // Multiple LPs propose
    propose_test_commitment(deps.as_mut(), mock_env(), "lp1").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp2").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp3").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp4").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp5").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp6").expect("should be able to propose");
    propose_test_commitment(deps.as_mut(), mock_env(), "lp7").expect("should be able to propose");

    // Accept 1,2,3
    accept_test_commitment(
        deps.as_mut(),
        mock_env(),
        "gp",
        &vec!["lp1", "lp2", "lp3", "lp7"],
    )
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

    deposit_test(
        deps.as_mut(),
        mock_env(),
        "lp7",
        &test_security_commitments(),
    )
    .expect("should be able to deposit partial amount");

    withdraw_test(deps.as_mut(), mock_env(), "gp", "lp7")
        .expect("should be able to withdraw full commitment");
}
