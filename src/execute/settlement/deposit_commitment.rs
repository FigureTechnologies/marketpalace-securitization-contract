use cosmwasm_std::{Addr, Coin, Response, Uint128};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        security::SecurityCommitment,
    },
    storage::{
        available_capital::{self},
        commits::{self},
        paid_in_capital::{self},
        securities::{self},
        state::{self},
    },
};

use super::commitment::CommitmentState;

pub fn handle(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    deposit: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if !is_accepted(&deps, &sender)? {
        return Err(crate::core::error::ContractError::InvalidCommitmentState {});
    }

    // We probably want to check that we don't over commit
    // If any are greater than the commitment
    // This could also be enforced by a rule that you must pay exactly x amount

    if !drawdown_met(&deps, &deposit) {
        return Err(crate::core::error::ContractError::InvalidSecurityCommitment {});
    }

    if !funds_match_deposit(&deps, &funds, &deposit, &state.capital_denom)? {
        return Err(crate::core::error::ContractError::FundMismatch {});
    }

    update_depositer_capital(deps, sender, funds, deposit)?;

    Ok(Response::default())
}

// This updates the AVAILABLE_CAPITAL and the PAID_IN_CAPITAL
fn update_depositer_capital(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    deposit: Vec<SecurityCommitment>,
) -> Result<(), ContractError> {
    paid_in_capital::add_payment(deps.storage, sender.clone(), deposit)?;
    available_capital::add_capital(deps.storage, sender, funds)?;
    Ok(())
}

// The purpose of this function is to make sure we have a valid drawdown.
// Check that the length is the same between the initial drawdown and our instatiation
// We check to make sure that every security commitment in the drawdown was specified at instantiation
// We also make sure our initial drawdown has the minimum for each of these security commitments
fn drawdown_met(deps: &ProvDepsMut, initial_drawdown: &Vec<SecurityCommitment>) -> bool {
    let security_types = securities::get_security_types(deps.storage);

    if security_types.len() != initial_drawdown.len() {
        return false;
    }

    for drawdown in initial_drawdown {
        let security = securities::get(deps.storage, drawdown.name.clone());
        if security.is_err() {
            return false;
        }
        let security = security.unwrap();
        if drawdown.amount < security.minimum_amount {
            return false;
        }
    }

    true
}

fn funds_match_deposit(
    deps: &ProvDepsMut,
    funds: &Vec<Coin>,
    deposit: &[SecurityCommitment],
    capital_denom: &String,
) -> Result<bool, ContractError> {
    let expected_funds = calculate_funds(deps, deposit, capital_denom)?;
    let has_funds = expected_funds.iter().all(|coin| funds.contains(coin));
    Ok(expected_funds.len() == funds.len() && has_funds)
}

// We are strict that all capital must be in the same denom
fn calculate_funds(
    deps: &ProvDepsMut,
    initial_drawdown: &[SecurityCommitment],
    capital_denom: &String,
) -> Result<Vec<Coin>, ContractError> {
    let mut sum = Coin::new(0, capital_denom);

    for security_commitment in initial_drawdown {
        let security = securities::get(deps.storage, security_commitment.name.clone())?;

        let cost = Uint128::from(security_commitment.amount) * security.price_per_unit.amount;
        sum.amount += cost;
    }

    Ok(vec![sum])
}

fn is_accepted(deps: &ProvDepsMut, sender: &Addr) -> Result<bool, ContractError> {
    let commitment = commits::get(deps.storage, sender.clone())?;
    Ok(commitment.state == CommitmentState::ACCEPTED)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::security::{FundSecurity, Security, SecurityCommitment, TrancheSecurity},
        execute::settlement::{
            commitment::{Commitment, CommitmentState},
            deposit_commitment::update_depositer_capital,
        },
        storage::{
            available_capital::{self},
            commits::{self},
            paid_in_capital::{self},
            securities::{self},
        },
        util::testing::SettlementTester,
    };

    use super::{calculate_funds, drawdown_met, funds_match_deposit, handle, is_accepted};

    #[test]
    fn test_is_accepted_throws_error_on_invalid_lp() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        is_accepted(&deps.as_mut(), &sender).unwrap_err();
    }

    #[test]
    fn test_is_accepted_should_return_false_on_invalid_state() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let commitment = Commitment::new(sender.clone(), vec![]);
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        let res = is_accepted(&deps.as_mut(), &sender).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_is_accepted_should_return_true_on_accepted() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut commitment = Commitment::new(sender.clone(), vec![]);
        commitment.state = CommitmentState::ACCEPTED;
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        let res = is_accepted(&deps.as_mut(), &sender).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_calculate_funds_should_throw_error_with_invalid_security() {
        let mut deps = mock_dependencies(&[]);
        let capital_denom = "denom".to_string();
        let securities = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: 5,
        }];

        calculate_funds(&deps.as_mut(), &securities, &capital_denom)
            .expect_err("should throw error");
    }

    #[test]
    fn test_calculate_funds_should_work_with_empty() {
        let mut deps = mock_dependencies(&[]);
        let capital_denom = "denom".to_string();
        let securities = vec![];

        let funds = calculate_funds(&deps.as_mut(), &securities, &capital_denom).unwrap();
        assert_eq!(vec![Coin::new(0, capital_denom)], funds);
    }

    #[test]
    fn test_caluclate_funds_works_with_multiple_securities() {
        let mut deps = mock_dependencies(&[]);
        let capital_denom = "denom".to_string();
        let securities = vec![
            Security {
                name: "Security1".to_string(),
                amount: 10,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(10, capital_denom.clone()),
            },
            Security {
                name: "Security2".to_string(),
                amount: 10,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(5, capital_denom.clone()),
            },
        ];
        let commitments = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 5,
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: 7,
            },
        ];
        securities::set(deps.as_mut().storage, &securities[0]).unwrap();
        securities::set(deps.as_mut().storage, &securities[1]).unwrap();

        let funds = calculate_funds(&deps.as_mut(), &commitments, &capital_denom).unwrap();
        assert_eq!(vec![Coin::new(85, capital_denom)], funds);
    }

    #[test]
    fn test_funds_match_deposit() {
        let mut deps = mock_dependencies(&[]);
        let capital_denom = "denom".to_string();
        let funds = vec![Coin::new(100, &capital_denom)];
        let deposit = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 1,
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: 2,
            },
        ];

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[0].name.clone(),
                amount: 10,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(50, &capital_denom),
            },
        )
        .unwrap();
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[1].name.clone(),
                amount: 10,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(25, &capital_denom),
            },
        )
        .unwrap();

        let res = funds_match_deposit(&deps.as_mut(), &funds, &deposit, &capital_denom).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_funds_match_deposit_should_fail_with_fund_amount_mismatch() {
        let mut deps = mock_dependencies(&[]);
        let capital_denom = "denom".to_string();
        let funds = vec![Coin::new(120, &capital_denom)];
        let deposit = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 1,
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: 2,
            },
        ];

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[0].name.clone(),
                amount: 10,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(50, &capital_denom),
            },
        )
        .unwrap();
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[1].name.clone(),
                amount: 10,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(25, &capital_denom),
            },
        )
        .unwrap();

        let res = funds_match_deposit(&deps.as_mut(), &funds, &deposit, &capital_denom).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_drawdown_met_can_handle_empty() {
        let mut deps = mock_dependencies(&[]);
        let initial_drawdown = vec![];
        let res = drawdown_met(&deps.as_mut(), &initial_drawdown);
        assert_eq!(true, res);
    }

    #[test]
    fn test_drawdown_security_length_doesnt_match() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![];
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: "Security1".to_string(),
                amount: 5,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(10, "denom".to_string()),
            },
        )
        .unwrap();
        let res = drawdown_met(&deps.as_mut(), &commitment);
        assert_eq!(false, res);
    }

    #[test]
    fn test_drawdown_invalid_security() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![SecurityCommitment {
            name: "Security2".to_string(),
            amount: 5,
        }];
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: "Security1".to_string(),
                amount: 5,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(10, "denom".to_string()),
            },
        )
        .unwrap();
        let res = drawdown_met(&deps.as_mut(), &commitment);
        assert_eq!(false, res);
    }

    #[test]
    fn test_drawdown_security_minimum_not_met() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: 1,
        }];
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: "Security1".to_string(),
                amount: 5,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 4,
                price_per_unit: Coin::new(10, "denom".to_string()),
            },
        )
        .unwrap();

        let res = drawdown_met(&deps.as_mut(), &commitment);
        assert_eq!(false, res);
    }

    #[test]
    fn test_drawdown_success() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: 1,
        }];
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: "Security1".to_string(),
                amount: 5,
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(10, "denom".to_string()),
            },
        )
        .unwrap();
        let res = drawdown_met(&deps.as_mut(), &commitment);
        assert_eq!(true, res);
    }

    #[test]
    fn test_update_depositer_capital_new_entry() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let capital_denom = "denom".to_string();
        let funds = vec![Coin::new(10, &capital_denom)];
        let deposit = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: 5,
        }];
        update_depositer_capital(deps.as_mut(), lp.clone(), funds.clone(), deposit.clone())
            .expect("should be successful");

        let paid_capital = paid_in_capital::get(&deps.storage, lp.clone()).unwrap();
        let available_capital = available_capital::get_capital(deps.as_mut().storage, lp).unwrap();

        assert_eq!(paid_capital, deposit);
        assert_eq!(available_capital, funds);
    }

    #[test]
    fn test_update_depositer_capital_update_entry() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let capital_denom = "denom".to_string();
        let funds = vec![Coin::new(10, &capital_denom)];
        let deposit = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: 5,
        }];

        paid_in_capital::set(deps.as_mut().storage, lp.clone(), &deposit).unwrap();
        available_capital::add_capital(deps.as_mut().storage, lp.clone(), funds.clone()).unwrap();

        update_depositer_capital(deps.as_mut(), lp.clone(), funds.clone(), deposit.clone())
            .expect("should be successful");

        let paid_capital = paid_in_capital::get(&deps.storage, lp.clone()).unwrap();
        let available_capital =
            available_capital::get_capital(deps.as_mut().storage, lp.clone()).unwrap();

        assert_eq!(
            paid_capital[0],
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 10,
            }
        );
        assert_eq!(available_capital[0], Coin::new(20, &capital_denom));
    }

    #[test]
    fn test_handle_should_throw_error_with_invalid_state() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let funds = vec![];
        let deposit = vec![];
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        let commitment = Commitment::new(sender.clone(), vec![]);
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        let error = handle(deps.as_mut(), sender, funds, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::InvalidCommitmentState {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_handle_should_throw_error_when_drawdown_not_met() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let funds = vec![Coin::new(10, "capital_denom".to_string())];
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let deposit = settlement_tester.security_commitments.clone();
        let mut commitment = Commitment::new(sender.clone(), vec![]);
        commitment.state = CommitmentState::ACCEPTED;

        settlement_tester.setup_test_state(deps.as_mut().storage);
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        let error = handle(deps.as_mut(), sender, funds, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::InvalidSecurityCommitment {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_handle_should_throw_error_when_funds_mismatch() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let funds = vec![Coin::new(10, "capital_denom".to_string())];
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        settlement_tester.setup_test_state(deps.as_mut().storage);
        let deposit = settlement_tester.security_commitments.clone();
        let mut commitment = Commitment::new(
            sender.clone(),
            settlement_tester.security_commitments.clone(),
        );
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: settlement_tester.security_commitments[0].name.clone(),
                amount: 100,
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(10, "capital_denom".to_string()),
            },
        )
        .unwrap();

        let error = handle(deps.as_mut(), sender, funds, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::FundMismatch {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_handle_should_work() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let funds = vec![Coin::new(
            settlement_tester.security_commitments[0].amount,
            "denom".to_string(),
        )];

        let deposit = settlement_tester.security_commitments.clone();
        let mut commitment = Commitment::new(
            sender.clone(),
            settlement_tester.security_commitments.clone(),
        );
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: settlement_tester.security_commitments[0].name.clone(),
                amount: 1000,
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: 1,
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        let response =
            handle(deps.as_mut(), sender, funds, deposit).expect("Should not throw error");
        assert_eq!(0, response.messages.len(),);
    }
}
