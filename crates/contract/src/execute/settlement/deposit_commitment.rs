use cosmwasm_std::{Addr, Coin, Env, Event, Response};
use provwasm_std::transfer_marker_coins;

use crate::storage::{securities, state};
use crate::{
    core::{
        aliases::{ProvDepsMut, ProvMsg, ProvTxResponse},
        error::ContractError,
        security::SecurityCommitment,
    },
    storage::{
        self,
        available_capital::{self},
        commits::{self},
        paid_in_capital::{self},
        // securities::{self},
    },
    util,
};

use super::commitment::CommitmentState;

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    deposit: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    let commitment = storage::commits::get(deps.storage, sender.clone())?;
    if util::settlement::is_expired(&env, &commitment) {
        return Err(crate::core::error::ContractError::SettlmentExpired {});
    }

    if !is_accepted(&deps, &sender)? {
        return Err(crate::core::error::ContractError::InvalidCommitmentState {});
    }

    if !securities_match(&deps, &deposit, sender.clone())? {
        return Err(crate::core::error::ContractError::InvalidSecurityCommitment {});
    }

    /*
    if !funds_match_deposit(&deps, &funds, &deposit, &state.capital_denom)? {
        return Err(crate::core::error::ContractError::FundMismatch {});
    }
    */

    if deposit_exceeds_commitment(&deps, sender.clone(), &deposit)? {
        return Err(crate::core::error::ContractError::ExcessiveDeposit {});
    }

    // convert the security commitment into actual fund coin
    // assumes for now that the deposit == commitment
    let funds = calculate_funds(&deps, &deposit, &state.capital_denom)?;
    let deposit_message =
        process_deposit(sender.clone(), env.contract.address.clone(), funds.clone())?;
    update_depositer_capital(deps, sender.clone(), funds, deposit)?;

    Ok(Response::new()
        .add_messages(deposit_message)
        .add_event(Event::new("deposited").add_attribute("lp", sender.clone()))
        .add_attribute("action", "deposit_commitment")
        .add_attribute("lp", sender.clone()))
}

// Check if they have a commitment - Shouldn't really matter
// What if they have no paid_capital?
fn deposit_exceeds_commitment(
    deps: &ProvDepsMut,
    lp: Addr,
    deposit: &[SecurityCommitment],
) -> Result<bool, ContractError> {
    let commitment = commits::get(deps.storage, lp.clone())?;
    let paid_capital = paid_in_capital::get(deps.storage, lp);

    // All new_capitals must be <= their respective security commitment
    let can_deposit = deposit.iter().all(|deposit_element| {
        // Get their previous payment4
        let paid_capital_element = paid_capital
            .iter()
            .find(|capital_element| capital_element.name == deposit_element.name);

        // Add any previous payments to their new payment
        let new_capital_amount = match paid_capital_element {
            None => deposit_element.amount,
            Some(element) => element.amount + deposit_element.amount,
        };

        // Find the element in commitment
        let security_commitment = commitment
            .commitments
            .iter()
            .find(|security_commitment| security_commitment.name == deposit_element.name);

        // If the security commitment doesn't exist then this deposit is invalid
        // The new_capital must <= security's commitment
        match security_commitment {
            None => false,
            Some(security_commitment) => new_capital_amount <= security_commitment.amount,
        }
    });

    Ok(!can_deposit)
}

fn process_deposit(
    sender: Addr,
    contract: Addr,
    funds: Vec<Coin>,
) -> Result<Vec<ProvMsg>, ContractError> {
    let mut messages = vec![];
    for fund in funds {
        if !fund.amount.is_zero() {
            messages.push(transfer_marker_coins(
                fund.amount.u128(),
                fund.denom,
                contract.clone(),
                sender.clone(),
            )?);
        }
    }

    Ok(messages)
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
fn securities_match(
    deps: &ProvDepsMut,
    deposit_securities: &[SecurityCommitment],
    lp: Addr,
) -> Result<bool, ContractError> {
    let commitment_securities: Vec<String> = commits::get(deps.storage, lp)?
        .commitments
        .iter()
        .map(|security| security.name.clone())
        .collect();

    Ok(deposit_securities
        .iter()
        .all(|deposit_security| commitment_securities.contains(&deposit_security.name)))
}
/*
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
*/
// We are strict that all capital must be in the same denom
fn calculate_funds(
    deps: &ProvDepsMut,
    deposit: &[SecurityCommitment],
    capital_denom: &String,
) -> Result<Vec<Coin>, ContractError> {
    let mut sum = Coin::new(0, capital_denom);

    for security_commitment in deposit {
        let security = securities::get(deps.storage, security_commitment.name.clone())?;

        let cost = security_commitment.amount * security.price_per_unit.amount;
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
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Coin, Uint128, Uint64};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::security::{FundSecurity, Security, SecurityCommitment, TrancheSecurity},
        execute::settlement::{
            commitment::{Commitment, CommitmentState},
            deposit_commitment::{deposit_exceeds_commitment, update_depositer_capital},
        },
        storage::{
            available_capital::{self},
            commits::{self},
            paid_in_capital::{self},
            securities::{self},
        },
        util::testing::SettlementTester,
    };

    // use super::{calculate_funds, funds_match_deposit, handle, is_accepted, securities_match};
    use super::{calculate_funds, handle, is_accepted, securities_match};

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
            amount: Uint128::new(5),
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
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(10, capital_denom.clone()),
            },
            Security {
                name: "Security2".to_string(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(5, capital_denom.clone()),
            },
        ];
        let commitments = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(7),
            },
        ];
        securities::set(deps.as_mut().storage, &securities[0]).unwrap();
        securities::set(deps.as_mut().storage, &securities[1]).unwrap();

        let funds = calculate_funds(&deps.as_mut(), &commitments, &capital_denom).unwrap();
        assert_eq!(vec![Coin::new(85, capital_denom)], funds);
    }

    /*
    #[test]
    fn test_funds_match_deposit() {
        let mut deps = mock_dependencies(&[]);
        let capital_denom = "denom".to_string();
        let funds = vec![Coin::new(100, &capital_denom)];
        let deposit = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(1),
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(2),
            },
        ];

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(50, &capital_denom),
            },
        )
        .unwrap();
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[1].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: Uint128::new(1),
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
                amount: Uint128::new(1),
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(2),
            },
        ];

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(50, &capital_denom),
            },
        )
        .unwrap();
        securities::set(
            deps.as_mut().storage,
            &Security {
                name: deposit[1].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(25, &capital_denom),
            },
        )
        .unwrap();

        let res = funds_match_deposit(&deps.as_mut(), &funds, &deposit, &capital_denom).unwrap();
        assert_eq!(false, res);
    }
    */

    #[test]
    fn test_securities_match_can_handle_empty() {
        let mut deps = mock_dependencies(&[]);
        let initial_drawdown = vec![];
        let lp = Addr::unchecked("lp");
        commits::set(
            deps.as_mut().storage,
            &Commitment {
                lp: lp.clone(),
                commitments: vec![SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: Uint128::new(5),
                }],
                state: CommitmentState::ACCEPTED,
                settlment_date: None,
            },
        )
        .unwrap();
        let res = securities_match(&deps.as_mut(), &initial_drawdown, lp).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_drawdown_security_length_doesnt_match() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        }];
        let lp = Addr::unchecked("lp");
        commits::set(
            deps.as_mut().storage,
            &Commitment {
                lp: lp.clone(),
                commitments: vec![
                    SecurityCommitment {
                        name: "Security1".to_string(),
                        amount: Uint128::new(5),
                    },
                    SecurityCommitment {
                        name: "Security2".to_string(),
                        amount: Uint128::new(5),
                    },
                ],
                state: CommitmentState::ACCEPTED,
                settlment_date: None,
            },
        )
        .unwrap();
        let res = securities_match(&deps.as_mut(), &commitment, lp).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_drawdown_invalid_security() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![
            SecurityCommitment {
                name: "Security3".to_string(),
                amount: Uint128::new(5),
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
        ];
        let lp = Addr::unchecked("lp");
        commits::set(
            deps.as_mut().storage,
            &Commitment {
                lp: lp.clone(),
                commitments: vec![
                    SecurityCommitment {
                        name: "Security1".to_string(),
                        amount: Uint128::new(5),
                    },
                    SecurityCommitment {
                        name: "Security2".to_string(),
                        amount: Uint128::new(5),
                    },
                ],
                state: CommitmentState::ACCEPTED,
                settlment_date: None,
            },
        )
        .unwrap();
        let res = securities_match(&deps.as_mut(), &commitment, lp).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_drawdown_success() {
        let mut deps = mock_dependencies(&[]);
        let commitment = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        }];
        let lp = Addr::unchecked("lp");
        commits::set(
            deps.as_mut().storage,
            &Commitment {
                lp: lp.clone(),
                commitments: vec![SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: Uint128::new(5),
                }],
                state: CommitmentState::ACCEPTED,
                settlment_date: None,
            },
        )
        .unwrap();
        let res = securities_match(&deps.as_mut(), &commitment, lp).unwrap();
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
            amount: Uint128::new(5),
        }];
        update_depositer_capital(deps.as_mut(), lp.clone(), funds.clone(), deposit.clone())
            .expect("should be successful");

        let paid_capital = paid_in_capital::get(&deps.storage, lp.clone());
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
            amount: Uint128::new(5),
        }];

        paid_in_capital::set(deps.as_mut().storage, lp.clone(), &deposit).unwrap();
        available_capital::add_capital(deps.as_mut().storage, lp.clone(), funds.clone()).unwrap();

        update_depositer_capital(deps.as_mut(), lp.clone(), funds.clone(), deposit.clone())
            .expect("should be successful");

        let paid_capital = paid_in_capital::get(&deps.storage, lp.clone());
        let available_capital =
            available_capital::get_capital(deps.as_mut().storage, lp.clone()).unwrap();

        assert_eq!(
            paid_capital[0],
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(10),
            }
        );
        assert_eq!(available_capital[0], Coin::new(20, &capital_denom));
    }

    #[test]
    fn test_handle_should_throw_error_with_invalid_state() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let deposit = vec![];
        let settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        let commitment = Commitment::new(sender.clone(), vec![]);
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        let error =
            handle(deps.as_mut(), mock_env(), sender, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::InvalidCommitmentState {}.to_string(),
            error.to_string()
        );
    }

    /*
    #[test]
    fn test_handle_should_throw_error_when_deposit_exceeds_commitment() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);


        let deposit = settlement_tester.security_commitments.clone();
        let mut commitment = Commitment::new(
            sender.clone(),
            settlement_tester.security_commitments.clone(),
        );
        commitment.state = CommitmentState::ACCEPTED;
        commitment.commitments[0].amount = Uint128::new(1);

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: settlement_tester.security_commitments[0].name.clone(),
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        let error =
            handle(deps.as_mut(), mock_env(), sender, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::ExcessiveDeposit {}.to_string(),
            error.to_string()
        );
    }
    */

    #[test]
    fn test_handle_should_throw_error_when_settlement_expired() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let funds = vec![Coin::new(
            settlement_tester.security_commitments[0].amount.u128(),
            "denom".to_string(),
        )];

        let deposit = settlement_tester.security_commitments.clone();
        let mut commitment = Commitment::new(
            sender.clone(),
            settlement_tester.security_commitments.clone(),
        );
        commitment.state = CommitmentState::ACCEPTED;
        commitment.settlment_date = Some(Uint64::new(mock_env().block.time.seconds() - 1));
        commitment.commitments[0].amount = Uint128::new(1);

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: settlement_tester.security_commitments[0].name.clone(),
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        let error =
            handle(deps.as_mut(), mock_env(), sender, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::SettlmentExpired {}.to_string(),
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

        let error =
            handle(deps.as_mut(), mock_env(), sender, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::InvalidSecurityCommitment {}.to_string(),
            error.to_string()
        );
    }

    /* Taken out because we don't check if funds match - because SC is pulling funds committed
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
                amount: Uint128::new(100),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(10, "capital_denom".to_string()),
            },
        )
        .unwrap();

        let error =
            handle(deps.as_mut(), mock_env(), sender, deposit).expect_err("should throw error");
        assert_eq!(
            crate::core::error::ContractError::FundMismatch {}.to_string(),
            error.to_string()
        );
    }
    */

    #[test]
    fn test_handle_should_work() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let funds = vec![Coin::new(
            settlement_tester.security_commitments[0].amount.u128(),
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
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        let response = handle(deps.as_mut(), mock_env(), sender.clone(), deposit)
            .expect("Should not throw error");
        assert_eq!(1, response.messages.len());
        assert_eq!(2, response.attributes.len());
        assert_eq!(
            Attribute::new("action", "deposit_commitment"),
            response.attributes[0]
        );
        assert_eq!(Attribute::new("lp", sender), response.attributes[1]);
    }

    #[test]
    fn test_handle_should_work_with_zero_on_a_security() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(2);
        settlement_tester.security_commitments[1].amount = Uint128::new(0);

        let mut deposit = settlement_tester.security_commitments.clone();
        deposit[1].amount = Uint128::new(0);
        let funds = vec![Coin::new(
            deposit[0].amount.u128() + deposit[1].amount.u128(),
            "denom".to_string(),
        )];

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
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: settlement_tester.security_commitments[1].name.clone(),
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(0),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        let response = handle(deps.as_mut(), mock_env(), sender.clone(), deposit)
            .expect("Should not throw error");
        assert_eq!(1, response.messages.len());
        assert_eq!(2, response.attributes.len());
        assert_eq!(
            Attribute::new("action", "deposit_commitment"),
            response.attributes[0]
        );
        assert_eq!(Attribute::new("lp", sender), response.attributes[1]);
    }

    #[test]
    fn test_handle_should_work_with_multiple() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(2);
        settlement_tester.security_commitments[1].amount = Uint128::new(0);

        let deposit = settlement_tester.security_commitments.clone();
        let funds = vec![Coin::new(
            deposit[0].amount.u128() + deposit[1].amount.u128(),
            "denom".to_string(),
        )];

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
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(1),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        securities::set(
            deps.as_mut().storage,
            &Security {
                name: settlement_tester.security_commitments[1].name.clone(),
                amount: Uint128::new(1000),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                minimum_amount: Uint128::new(0),
                price_per_unit: Coin::new(1, "denom".to_string()),
            },
        )
        .unwrap();

        let response = handle(deps.as_mut(), mock_env(), sender.clone(), deposit)
            .expect("Should not throw error");
        assert_eq!(1, response.messages.len());
        assert_eq!(2, response.attributes.len());
        assert_eq!(
            Attribute::new("action", "deposit_commitment"),
            response.attributes[0]
        );
        assert_eq!(Attribute::new("lp", sender), response.attributes[1]);
    }

    #[test]
    fn test_deposit_exceeds_commitment_throws_error_on_invalid_lp() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let deposit = Vec::<SecurityCommitment>::new();
        deposit_exceeds_commitment(&deps.as_mut(), lp, &deposit)
            .expect_err("should throw error on invalid lp");
    }

    #[test]
    fn test_deposit_exceeds_commitment_should_succeed_on_empty_deposit() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        let deposit = Vec::<SecurityCommitment>::new();
        deposit_exceeds_commitment(&deps.as_mut(), commitment.lp, &deposit)
            .expect("should not throw an error");
    }

    #[test]
    fn test_deposit_exceeds_commitment_should_fail_on_invalid_security() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        settlement_tester.create_security_commitments(1);
        let deposit = settlement_tester.security_commitments.clone();
        let res = deposit_exceeds_commitment(&deps.as_mut(), commitment.lp, &deposit).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_deposit_exceeds_commitment_should_fail_on_initial_deposit_that_exceeds() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        let mut deposit = settlement_tester.security_commitments.clone();
        deposit[0].amount += Uint128::new(1);
        let res = deposit_exceeds_commitment(&deps.as_mut(), commitment.lp, &deposit).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_deposit_exceeds_commitment_should_fail_on_additional_deposit_that_exceeds() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments,
        )
        .unwrap();

        let deposit = settlement_tester.security_commitments.clone();
        let res =
            deposit_exceeds_commitment(&deps.as_mut(), commitment.lp.clone(), &deposit).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_deposit_exceeds_commitment_should_succeed_on_valid_initial_deposit() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        let deposit = settlement_tester.security_commitments.clone();
        let res =
            deposit_exceeds_commitment(&deps.as_mut(), commitment.lp.clone(), &deposit).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_deposit_exceeds_commitment_should_succeed_on_additional_deposit() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();

        let mut small_deposit = settlement_tester.security_commitments.clone();
        small_deposit[0].amount = Uint128::new(1);

        paid_in_capital::set(deps.as_mut().storage, commitment.lp.clone(), &small_deposit).unwrap();

        let res = deposit_exceeds_commitment(&deps.as_mut(), commitment.lp.clone(), &small_deposit)
            .unwrap();
        assert_eq!(false, res);
    }
}
