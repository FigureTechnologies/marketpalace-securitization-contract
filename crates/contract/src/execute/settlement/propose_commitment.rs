use cosmwasm_std::{Addr, Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        security::SecurityCommitment,
    },
    storage::{
        commits::{self},
        remaining_securities,
        securities::{self},
    },
    util::settlement::timestamp_is_expired,
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(
    deps: ProvDepsMut,
    env: &Env,
    lp: Addr,
    securities: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let commitment = commits::get(deps.storage, lp.clone());

    if is_approved(&commitment) {
        return Err(crate::core::error::ContractError::AlreadyAccepted {});
    }

    if !is_new_securities(&commitment, &securities) {
        return Err(crate::core::error::ContractError::InvalidSecurityCommitment {});
    }

    if timestamp_is_expired(deps.storage, &env.block.time)? {
        return Err(crate::core::error::ContractError::SettlmentExpired {});
    }

    for security_commitment in &securities {
        let security = securities::get(deps.storage, security_commitment.name.clone())?;
        if security_commitment.amount < security.minimum_amount {
            return Err(crate::core::error::ContractError::InvalidSecurityCommitmentAmount {});
        }
        if !remaining_securities::has_amount(
            deps.storage,
            security_commitment.name.clone(),
            security_commitment.amount.u128(),
        )? {
            return Err(
                crate::core::error::ContractError::CommitmentExceedsRemainingSecurityAmount {},
            );
        }
    }

    let mut new_commitment = Commitment::new(lp.clone(), securities);
    if let Ok(mut commitment) = commitment {
        new_commitment
            .commitments
            .append(&mut commitment.commitments)
    }
    commits::set(deps.storage, &new_commitment)?;

    Ok(Response::new()
        .add_attribute("action", "propose_commitment")
        .add_attribute("lp", lp))
}

fn is_approved(commitment: &Result<Commitment, ContractError>) -> bool {
    if commitment.is_err() {
        return false;
    }
    let commitment = commitment.as_ref().unwrap();
    commitment.state != CommitmentState::PENDING
}

fn is_new_securities(
    commitment: &Result<Commitment, ContractError>,
    securities: &[SecurityCommitment],
) -> bool {
    if commitment.is_err() {
        return true;
    }
    let new_names: Vec<String> = securities
        .iter()
        .map(|security| security.name.clone())
        .collect();
    commitment
        .as_ref()
        .unwrap()
        .commitments
        .iter()
        .all(|security| !new_names.contains(&security.name))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Coin, Uint128};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::{
            error::ContractError,
            security::{FundSecurity, Security, SecurityCommitment},
        },
        execute::{
            settlement::commitment::CommitmentState,
            settlement::{
                commitment::Commitment,
                propose_commitment::{handle, is_approved},
            },
        },
        storage::{
            commits::{self},
            remaining_securities,
            securities::{self},
        },
        util::testing::{create_test_state, SettlementTester},
    };

    use super::is_new_securities;

    #[test]
    fn test_is_approved_is_false_for_error_result() {
        let result = is_approved(&Err(ContractError::Unauthorized {}));
        assert_eq!(false, result);
    }

    #[test]
    fn test_is_approved_is_false_for_pending() {
        let old_securities = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        }];
        let result = is_approved(&Ok(Commitment::new(Addr::unchecked("lp"), old_securities)));
        assert_eq!(false, result);
    }

    #[test]
    fn test_is_approved_is_true_for_not_pending() {
        let old_securities = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        }];
        let mut commitment = Commitment::new(Addr::unchecked("lp"), old_securities);
        commitment.state = CommitmentState::SETTLED;
        let result = is_approved(&Ok(commitment.clone()));
        assert_eq!(true, result);
        commitment.state = CommitmentState::ACCEPTED;
        let result = is_approved(&Ok(commitment));
        assert_eq!(true, result);
    }

    #[test]
    fn test_is_new_securities_is_true_for_error_result() {
        let result = is_new_securities(&Err(ContractError::Unauthorized {}), &[]);
        assert_eq!(true, result);
    }

    #[test]
    fn test_is_new_securities_is_true_for_new_securities() {
        let old_securities = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        }];
        let new_securities = vec![
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(5),
            },
            SecurityCommitment {
                name: "Security3".to_string(),
                amount: Uint128::new(5),
            },
        ];
        let result = is_new_securities(
            &Ok(Commitment::new(Addr::unchecked("lp"), old_securities)),
            &new_securities,
        );
        assert_eq!(true, result);
    }

    #[test]
    fn test_is_new_securities_is_false_for_existing_securities() {
        let old_securities = vec![SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        }];
        let new_securities = vec![
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(5),
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
        ];
        let result = is_new_securities(
            &Ok(Commitment::new(Addr::unchecked("lp"), old_securities)),
            &new_securities,
        );
        assert_eq!(false, result);
    }

    #[test]
    fn test_minimums_are_met() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        create_test_state(&mut deps, &mock_env(), false);
        let commitments = settlement_tester.security_commitments.clone();
        securities::set(
            &mut deps.storage,
            &Security {
                name: commitments[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: commitments[0].amount + Uint128::new(1),
                price_per_unit: Coin::new(5, "denom".to_string()),
            },
        )
        .unwrap();
        let res = handle(deps.as_mut(), &mock_env(), lp, commitments).unwrap_err();

        assert_eq!(
            ContractError::InvalidSecurityCommitmentAmount {}.to_string(),
            res.to_string()
        )
    }

    #[test]
    fn test_all_securities_exist() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        create_test_state(&mut deps, &mock_env(), false);
        settlement_tester.create_security_commitments(1);
        let commitments = settlement_tester.security_commitments.clone();
        handle(deps.as_mut(), &mock_env(), lp, commitments).unwrap_err();
    }

    #[test]
    fn test_fails_on_expired_timestamp() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let mut env = mock_env();
        create_test_state(&mut deps, &env, true);
        env.block.time = env.block.time.plus_seconds(86401);
        let commitments = settlement_tester.security_commitments.clone();
        securities::set(
            &mut deps.storage,
            &Security {
                name: commitments[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: commitments[0].amount,
                price_per_unit: Coin::new(5, "denom".to_string()),
            },
        )
        .unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            commitments[0].name.clone(),
            commitments[0].amount.u128(),
        )
        .unwrap();
        let err = handle(deps.as_mut(), &env, lp.clone(), commitments.clone()).unwrap_err();
        assert_eq!(
            ContractError::SettlmentExpired {}.to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_commit_is_added_on_success_with_unexpired_timestamp() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        create_test_state(&mut deps, &mock_env(), true);
        let commitments = settlement_tester.security_commitments.clone();
        securities::set(
            &mut deps.storage,
            &Security {
                name: commitments[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: commitments[0].amount,
                price_per_unit: Coin::new(5, "denom".to_string()),
            },
        )
        .unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            commitments[0].name.clone(),
            commitments[0].amount.u128(),
        )
        .unwrap();
        let res = handle(deps.as_mut(), &mock_env(), lp.clone(), commitments.clone()).unwrap();

        let commitment = commits::get(&deps.storage, lp.clone()).unwrap();
        assert_eq!(commitments, commitment.commitments);
        assert_eq!(CommitmentState::PENDING, commitment.state);
        assert_eq!(lp, commitment.lp);
        assert_eq!(2, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "propose_commitment"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("lp", lp), res.attributes[1]);
    }

    #[test]
    fn test_commit_is_added_on_success() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        create_test_state(&mut deps, &mock_env(), false);
        let commitments = settlement_tester.security_commitments.clone();
        securities::set(
            &mut deps.storage,
            &Security {
                name: commitments[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: commitments[0].amount,
                price_per_unit: Coin::new(5, "denom".to_string()),
            },
        )
        .unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            commitments[0].name.clone(),
            commitments[0].amount.u128(),
        )
        .unwrap();
        let res = handle(deps.as_mut(), &mock_env(), lp.clone(), commitments.clone()).unwrap();

        let commitment = commits::get(&deps.storage, lp.clone()).unwrap();
        assert_eq!(commitments, commitment.commitments);
        assert_eq!(CommitmentState::PENDING, commitment.state);
        assert_eq!(lp, commitment.lp);
        assert_eq!(2, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "propose_commitment"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("lp", lp), res.attributes[1]);
    }

    #[test]
    fn test_cannot_accept_security_when_total_supply_is_greater_than_amount() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        create_test_state(&mut deps, &mock_env(), false);
        let commitments = settlement_tester.security_commitments.clone();
        securities::set(
            &mut deps.storage,
            &Security {
                name: commitments[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: commitments[0].amount,
                price_per_unit: Coin::new(5, "denom".to_string()),
            },
        )
        .unwrap();
        let error =
            handle(deps.as_mut(), &mock_env(), lp.clone(), commitments.clone()).unwrap_err();
        assert_eq!(
            ContractError::CommitmentExceedsRemainingSecurityAmount {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_cannot_double_commit_same_security() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        create_test_state(&mut deps, &mock_env(), false);
        let commitments = settlement_tester.security_commitments.clone();
        securities::set(
            &mut deps.storage,
            &Security {
                name: commitments[0].name.clone(),
                amount: Uint128::new(10),
                security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                minimum_amount: commitments[0].amount,
                price_per_unit: Coin::new(5, "denom".to_string()),
            },
        )
        .unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            commitments[0].name.clone(),
            commitments[0].amount.u128(),
        )
        .unwrap();
        handle(deps.as_mut(), &mock_env(), lp.clone(), commitments.clone()).unwrap();
        let res = handle(deps.as_mut(), &mock_env(), lp.clone(), commitments.clone()).unwrap_err();
        assert_eq!(
            ContractError::InvalidSecurityCommitment {}.to_string(),
            res.to_string()
        );
    }

    #[test]
    fn test_can_double_commit_on_different_securities() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");

        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        create_test_state(&mut deps, &mock_env(), false);
        for security_commitment in &settlement_tester.security_commitments {
            securities::set(
                &mut deps.storage,
                &Security {
                    name: security_commitment.name.clone(),
                    amount: Uint128::new(10),
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                    minimum_amount: Uint128::zero(),
                    price_per_unit: Coin::new(5, "denom".to_string()),
                },
            )
            .unwrap();
            remaining_securities::set(
                deps.as_mut().storage,
                security_commitment.name.clone(),
                security_commitment.amount.u128(),
            )
            .unwrap();
        }

        let commitment1 = vec![settlement_tester.security_commitments[0].clone()];
        let commitment2 = vec![settlement_tester.security_commitments[1].clone()];

        handle(deps.as_mut(), &mock_env(), lp.clone(), commitment2.clone()).unwrap();
        handle(deps.as_mut(), &mock_env(), lp.clone(), commitment1.clone()).unwrap();

        let commitment = commits::get(&deps.storage, lp.clone()).unwrap();
        assert_eq!(
            settlement_tester.security_commitments,
            commitment.commitments
        );
        assert_eq!(CommitmentState::PENDING, commitment.state);
        assert_eq!(lp, commitment.lp);
    }

    #[test]
    fn test_can_only_commit_on_pending_securities() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");

        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        create_test_state(&mut deps, &mock_env(), false);
        for security_commitment in &settlement_tester.security_commitments {
            securities::set(
                &mut deps.storage,
                &Security {
                    name: security_commitment.name.clone(),
                    amount: Uint128::new(10),
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                    minimum_amount: Uint128::zero(),
                    price_per_unit: Coin::new(5, "denom".to_string()),
                },
            )
            .unwrap();
            remaining_securities::set(
                deps.as_mut().storage,
                security_commitment.name.clone(),
                security_commitment.amount.u128(),
            )
            .unwrap();
        }

        let commitment1 = vec![settlement_tester.security_commitments[0].clone()];
        let commitment2 = vec![settlement_tester.security_commitments[1].clone()];

        handle(deps.as_mut(), &mock_env(), lp.clone(), commitment2.clone()).unwrap();

        // Here we want to change the state
        let mut commitment = commits::get(&deps.storage, lp.clone()).unwrap();
        commitment.state = CommitmentState::ACCEPTED;
        commits::set(&mut deps.storage, &commitment).unwrap();

        let err = handle(deps.as_mut(), &mock_env(), lp.clone(), commitment1.clone()).unwrap_err();
        assert_eq!(
            ContractError::AlreadyAccepted {}.to_string(),
            err.to_string()
        );
    }
}
