use cosmwasm_std::{Addr, Coin, Order, Response, StdResult, Uint128};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        security::SecurityCommitment,
    },
    storage::{
        available_capital::AVAILABLE_CAPITAL, commits::COMMITS, paid_in_capital::PAID_IN_CAPITAL,
        securities::SECURITIES_MAP, state::STATE,
    },
};

use super::commitment::CommitmentState;

pub fn handle(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    deposit: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
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
    PAID_IN_CAPITAL.update(
        deps.storage,
        sender.clone(),
        |already_committed| -> StdResult<Vec<SecurityCommitment>> {
            match already_committed {
                None => Ok(deposit),
                Some(mut already_committed) => {
                    for deposit_security in &deposit {
                        add_security_commitment(deposit_security, &mut already_committed);
                    }
                    Ok(already_committed)
                }
            }
        },
    )?;

    AVAILABLE_CAPITAL.update(
        deps.storage,
        sender,
        |available_capital| -> StdResult<Vec<Coin>> {
            match available_capital {
                None => Ok(funds),
                Some(mut available_capital) => {
                    for fund_coin in &funds {
                        add_to_capital(fund_coin, &mut available_capital);
                    }
                    Ok(available_capital)
                }
            }
        },
    )?;

    Ok(())
}

// The purpose of this function is to make sure we have a valid drawdown.
// Check that the length is the same between the initial drawdown and our instatiation
// We check to make sure that every security commitment in the drawdown was specified at instantiation
// We also make sure our initial drawdown has the minimum for each of these security commitments
fn drawdown_met(deps: &ProvDepsMut, initial_drawdown: &Vec<SecurityCommitment>) -> bool {
    let security_types: StdResult<Vec<_>> = SECURITIES_MAP
        .keys(deps.storage, None, None, Order::Ascending)
        .collect();
    let security_types = security_types.unwrap();

    if security_types.len() != initial_drawdown.len() {
        return false;
    }

    for drawdown in initial_drawdown {
        let security = SECURITIES_MAP.load(deps.storage, drawdown.name.clone());
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
        let security = SECURITIES_MAP.load(deps.storage, security_commitment.name.clone())?;

        let cost = Uint128::from(security_commitment.amount) * security.price_per_unit.amount;
        sum.amount += cost;
    }

    Ok(vec![sum])
}

fn is_accepted(deps: &ProvDepsMut, sender: &Addr) -> Result<bool, ContractError> {
    let commitment = COMMITS.load(deps.storage, sender.clone())?;
    Ok(commitment.state == CommitmentState::ACCEPTED)
}

// The purpose of this function is to add new_commitment to commitments.
// We do this by finding the security commitment that has the same name as new_commitment,
// and then we add the new_commitment.amount to the commitment.amount.
//
// Note this modifies commitments
fn add_security_commitment(
    new_commitment: &SecurityCommitment,
    commitments: &mut [SecurityCommitment],
) {
    for commitment in commitments.iter_mut() {
        if commitment.name == new_commitment.name {
            commitment.amount += new_commitment.amount;
            break;
        }
    }
}

// The purpose of this function is to add a coin to capital.
// We do this by finding the coin that has the same name as new_coin,
// and then we add the new_coin.amount to the coin.amount.
//
// Note this modifies capital
fn add_to_capital(new_coin: &Coin, capital: &mut [Coin]) {
    for coin in capital.iter_mut() {
        if coin.denom == new_coin.denom {
            coin.amount += new_coin.amount;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::security::{self, FundSecurity, Security, SecurityCommitment, TrancheSecurity},
        execute::settlement::{
            self,
            commitment::{Commitment, CommitmentState},
            deposit_commitment::{add_security_commitment, update_depositer_capital},
        },
        storage::{
            available_capital::AVAILABLE_CAPITAL, commits::COMMITS,
            paid_in_capital::PAID_IN_CAPITAL, securities::SECURITIES_MAP,
        },
        util::{self, testing::SettlementTester},
    };

    use super::{
        add_to_capital, calculate_funds, drawdown_met, funds_match_deposit, handle, is_accepted,
    };

    #[test]
    fn test_add_to_capital_works_with_empty() {
        let denom = "denom".to_string();
        let coin = Coin::new(100, denom);
        let mut capital = vec![];
        add_to_capital(&coin, &mut capital);

        assert_eq!(0, capital.len());
    }

    #[test]
    fn test_add_to_capital_updates_first_capital() {
        let denom = "denom".to_string();
        let coin = Coin::new(100, denom.clone());
        let mut capital = vec![Coin::new(100, denom.clone()), Coin::new(100, denom.clone())];
        add_to_capital(&coin, &mut capital);

        assert_eq!(2, capital.len());
        assert_eq!(Coin::new(200, denom.clone()), capital[0]);
        assert_eq!(Coin::new(100, denom.clone()), capital[1]);
    }

    #[test]
    fn test_add_to_capital_ignores_invalid_coin() {
        let denom = "denom".to_string();
        let denom2 = "denom2".to_string();
        let coin = Coin::new(100, denom.clone());
        let mut capital = vec![
            Coin::new(100, denom2.clone()),
            Coin::new(100, denom.clone()),
        ];
        add_to_capital(&coin, &mut capital);

        assert_eq!(2, capital.len());
        assert_eq!(Coin::new(100, denom2.clone()), capital[0]);
        assert_eq!(Coin::new(200, denom.clone()), capital[1]);
    }

    #[test]
    fn test_add_security_commitment_with_empty() {
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let new_commitment = settlement_tester.security_commitments[0].clone();
        let mut commitments = vec![];

        add_security_commitment(&new_commitment, &mut commitments);
        assert_eq!(0, commitments.len());
    }

    #[test]
    fn test_add_security_commitment_updates_first_capital() {
        let new_commitment = SecurityCommitment {
            name: "Security1".to_string(),
            amount: 5,
        };
        let mut commitments = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 7,
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 5,
            },
        ];

        add_security_commitment(&new_commitment, &mut commitments);
        assert_eq!(2, commitments.len());
        assert_eq!(12, commitments[0].amount);
        assert_eq!(5, commitments[1].amount);
    }

    #[test]
    fn test_add_security_commitment_ignores_invalid_name() {
        let new_commitment = SecurityCommitment {
            name: "Security1".to_string(),
            amount: 5,
        };
        let mut commitments = vec![
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: 7,
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: 5,
            },
        ];

        add_security_commitment(&new_commitment, &mut commitments);
        assert_eq!(2, commitments.len());
        assert_eq!(7, commitments[0].amount);
        assert_eq!(10, commitments[1].amount);
    }

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
        COMMITS
            .save(
                deps.as_mut().storage,
                sender.clone(),
                &Commitment::new(sender.clone(), vec![]),
            )
            .unwrap();
        let res = is_accepted(&deps.as_mut(), &sender).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_is_accepted_should_return_true_on_accepted() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut commitment = Commitment::new(sender.clone(), vec![]);
        commitment.state = CommitmentState::ACCEPTED;
        COMMITS
            .save(deps.as_mut().storage, sender.clone(), &commitment)
            .unwrap();
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
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                securities[0].name.clone(),
                &securities[0],
            )
            .unwrap();
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                securities[1].name.clone(),
                &securities[1],
            )
            .unwrap();

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

        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                deposit[0].name.clone(),
                &Security {
                    name: deposit[0].name.clone(),
                    amount: 10,
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                    minimum_amount: 1,
                    price_per_unit: Coin::new(50, &capital_denom),
                },
            )
            .unwrap();
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                deposit[1].name.clone(),
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

        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                deposit[0].name.clone(),
                &Security {
                    name: deposit[0].name.clone(),
                    amount: 10,
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                    minimum_amount: 1,
                    price_per_unit: Coin::new(50, &capital_denom),
                },
            )
            .unwrap();
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                deposit[1].name.clone(),
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
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                "Security1".to_string(),
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
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                "Security1".to_string(),
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
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                "Security1".to_string(),
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
        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                "Security1".to_string(),
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

        let paid_capital = PAID_IN_CAPITAL
            .load(deps.as_mut().storage, lp.clone())
            .unwrap();
        let available_capital = AVAILABLE_CAPITAL
            .load(deps.as_mut().storage, lp.clone())
            .unwrap();

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

        PAID_IN_CAPITAL
            .save(deps.as_mut().storage, lp.clone(), &deposit)
            .unwrap();
        AVAILABLE_CAPITAL
            .save(deps.as_mut().storage, lp.clone(), &funds)
            .unwrap();

        update_depositer_capital(deps.as_mut(), lp.clone(), funds.clone(), deposit.clone())
            .expect("should be successful");

        let paid_capital = PAID_IN_CAPITAL
            .load(deps.as_mut().storage, lp.clone())
            .unwrap();
        let available_capital = AVAILABLE_CAPITAL
            .load(deps.as_mut().storage, lp.clone())
            .unwrap();

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
        COMMITS
            .save(
                deps.as_mut().storage,
                Addr::unchecked("lp"),
                &Commitment::new(sender.clone(), vec![]),
            )
            .unwrap();

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
        COMMITS
            .save(deps.as_mut().storage, Addr::unchecked("lp"), &commitment)
            .unwrap();

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

        COMMITS
            .save(deps.as_mut().storage, Addr::unchecked("lp"), &commitment)
            .unwrap();

        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                settlement_tester.security_commitments[0].name.clone(),
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

        COMMITS
            .save(deps.as_mut().storage, Addr::unchecked("lp"), &commitment)
            .unwrap();

        SECURITIES_MAP
            .save(
                deps.as_mut().storage,
                settlement_tester.security_commitments[0].name.clone(),
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
