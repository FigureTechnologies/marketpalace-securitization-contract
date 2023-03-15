use cosmwasm_std::{Addr, Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvResponse, ProvTxResponse},
        error::ContractError,
    },
    storage::{self, state},
    util,
};

use super::{commitment::CommitmentState, withdraw_commitment};

pub fn handle(mut deps: ProvDepsMut, env: Env, sender: Addr) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        return Err(ContractError::Unauthorized {});
    }

    if util::settlement::timestamp_is_expired(deps.storage, &env.block.time)? {
        return Err(ContractError::SettlmentExpired {});
    }

    let commits = storage::commits::get_with_state(deps.storage, CommitmentState::ACCEPTED);
    let mut res = Response::new()
        .add_attribute("action", "withdraw_all_commitments")
        .add_attribute("gp", sender.clone());
    for commit in commits {
        if let Ok(withdraw) = withdraw_commitment::withdraw_commitment(
            &mut deps,
            &env,
            sender.clone(),
            commit.lp.clone(),
        ) {
            res = res.add_submessages(withdraw.messages);
            res = res.add_events(withdraw.events);
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::error::ContractError,
        execute::settlement::{
            commitment::{Commitment, CommitmentState},
            withdraw_all_commitments::handle,
        },
        storage::{available_capital, commits, paid_in_capital},
        util::testing::{create_test_state, SettlementTester},
    };

    #[test]
    fn test_should_fail_when_sender_must_be_gp() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");

        let settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);

        let error = handle(deps.as_mut(), mock_env(), sender.clone()).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_should_fail_when_settlement_is_expired() {
        let mut deps = mock_dependencies(&[]);
        let mut env = mock_env();
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(2);
        create_test_state(&mut deps, &env, true);
        env.block.time = env.block.time.plus_seconds(86401);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let capital_denom = "denom".to_string();
        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(100, &capital_denom)],
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments.clone(),
        )
        .unwrap();

        let err = handle(deps.as_mut(), env, gp.clone()).unwrap_err();
        assert_eq!(
            ContractError::SettlmentExpired {}.to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_should_succeed_when_settlement_time_does_not_exist() {
        let mut deps = mock_dependencies(&[]);

        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let capital_denom = "denom".to_string();
        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(100, &capital_denom)],
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments.clone(),
        )
        .unwrap();

        let res = handle(deps.as_mut(), mock_env(), gp.clone()).unwrap();
        assert_eq!(1, res.events.len());
        assert_eq!(
            vec![
                Attribute::new("action", "withdraw_all_commitments"),
                Attribute::new("gp", gp)
            ],
            res.attributes
        );
    }

    #[test]
    fn test_should_succeed_when_settlement_time_is_not_expired() {
        let mut deps = mock_dependencies(&[]);

        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(2);
        create_test_state(&mut deps, &mock_env(), true);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let capital_denom = "denom".to_string();
        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(100, &capital_denom)],
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments.clone(),
        )
        .unwrap();

        let res = handle(deps.as_mut(), mock_env(), gp.clone()).unwrap();
        assert_eq!(1, res.events.len());
        assert_eq!(
            vec![
                Attribute::new("action", "withdraw_all_commitments"),
                Attribute::new("gp", gp)
            ],
            res.attributes
        );
    }

    #[test]
    fn test_should_succeed_with_multiple() {
        let mut deps = mock_dependencies(&[]);

        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let lp2 = Addr::unchecked("lp2");
        let gp = Addr::unchecked("gp");
        let capital_denom = "denom".to_string();

        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;
        let mut commitment2 = Commitment::new(lp2, settlement_tester.security_commitments.clone());
        commitment2.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();
        commits::set(deps.as_mut().storage, &commitment2).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(100, &capital_denom)],
        )
        .unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment2.lp.clone(),
            vec![Coin::new(100, &capital_denom)],
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments.clone(),
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment2.lp.clone(),
            &settlement_tester.security_commitments.clone(),
        )
        .unwrap();

        let res = handle(deps.as_mut(), mock_env(), gp.clone()).unwrap();
        assert_eq!(2, res.events.len());
        assert_eq!(
            vec![
                Attribute::new("action", "withdraw_all_commitments"),
                Attribute::new("gp", gp)
            ],
            res.attributes
        );
    }
}
