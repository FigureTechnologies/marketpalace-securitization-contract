use cosmwasm_std::{Addr, Env, Event, Response, Storage};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::{
        commits::{self},
        paid_in_capital::{self},
        remaining_securities,
        state::{self},
    },
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(
    deps: ProvDepsMut,
    _env: Env,
    sender: Addr,
    commitments: Vec<Addr>,
) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        return Err(crate::core::error::ContractError::Unauthorized {});
    }

    let mut response = Response::new()
        .add_attribute("action", "accept_commitments")
        .add_attribute("gp", state.gp);
    for lp in commitments {
        accept_commitment(deps.storage, lp.clone())?;
        response = response.add_event(Event::new("accepted").add_attribute("lp", lp));
    }

    Ok(response)
}

fn accept_commitment(storage: &mut dyn Storage, lp: Addr) -> Result<(), ContractError> {
    let mut commitment = commits::get(storage, lp)?;

    if commitment.state != CommitmentState::PENDING {
        return Err(ContractError::InvalidCommitmentState {});
    }

    // Remove from remaining
    for security_commitment in &commitment.commitments {
        if !remaining_securities::subtract(
            storage,
            security_commitment.name.clone(),
            security_commitment.amount.u128(),
        )? {
            return Err(
                crate::core::error::ContractError::CommitmentExceedsRemainingSecurityAmount {},
            );
        }
    }

    commitment.state = CommitmentState::ACCEPTED;
    commitment.settlment_date = state::get_settlement_time(storage)?;
    commits::set(storage, &commitment)?;

    track_paid_capital(storage, commitment)?;
    Ok(())
}

fn track_paid_capital(
    storage: &mut dyn Storage,
    mut commitment: Commitment,
) -> Result<(), ContractError> {
    commitment.clear_amounts();
    paid_in_capital::set(storage, commitment.lp, &commitment.commitments)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::error::ContractError,
        execute::settlement::commitment::{Commitment, CommitmentState},
        storage::{
            commits::{self},
            paid_in_capital::{self},
            remaining_securities,
            state::{self, State},
        },
        util::testing::{create_test_state, SettlementTester},
    };

    use super::{accept_commitment, handle, track_paid_capital};

    #[test]
    fn test_accepted_commit_must_exist() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let res = accept_commitment(deps.as_mut().storage, lp);
        res.expect_err("should have error for invalid commit");
    }

    #[test]
    fn test_accepted_commit_must_be_pending() {
        let lp = Addr::unchecked("address");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let mut commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        let error = accept_commitment(deps.as_mut().storage, lp).unwrap_err();
        assert_eq!(
            ContractError::InvalidCommitmentState {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_accepted_commit_cannot_make_sum_of_securities_greater_than_the_amount() {
        let lp = Addr::unchecked("address");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            settlement_tester.security_commitments[0].name.clone(),
            settlement_tester.security_commitments[0].amount.u128() - 1,
        )
        .unwrap();
        let error = accept_commitment(deps.as_mut().storage, lp.clone()).unwrap_err();

        assert_eq!(
            ContractError::CommitmentExceedsRemainingSecurityAmount {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_track_paid_capital_makes_an_empty_entry() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());

        track_paid_capital(deps.as_mut().storage, commitment.clone()).unwrap();
        let paid_capital = paid_in_capital::get(&deps.storage, commitment.lp);
        for capital in &paid_capital {
            assert_eq!(0, capital.amount.u128());
        }
    }

    #[test]
    fn test_accept_commit_succeeds_and_updates_settlement_time() {
        let lp = Addr::unchecked("address");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        create_test_state(&mut deps, true);
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            settlement_tester.security_commitments[0].name.clone(),
            settlement_tester.security_commitments[0].amount.u128(),
        )
        .unwrap();
        accept_commitment(deps.as_mut().storage, lp.clone()).unwrap();

        // We need to check the state
        let added_commitment = commits::get(&deps.storage, lp).unwrap();
        let expected_time = state::get_settlement_time(&deps.storage).unwrap();
        assert_eq!(CommitmentState::ACCEPTED, added_commitment.state);
        assert_eq!(expected_time, added_commitment.settlment_date);

        // We need to check capital
        let paid_capital = paid_in_capital::get(&deps.storage, commitment.lp);
        for capital in &paid_capital {
            assert_eq!(0, capital.amount.u128());
        }
    }

    #[test]
    fn test_accept_commit_succeeds() {
        let lp = Addr::unchecked("address");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        create_test_state(&mut deps, false);
        settlement_tester.create_security_commitments(1);
        let commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            settlement_tester.security_commitments[0].name.clone(),
            settlement_tester.security_commitments[0].amount.u128(),
        )
        .unwrap();
        accept_commitment(deps.as_mut().storage, lp.clone()).unwrap();

        // We need to check the state
        let added_commitment = commits::get(&deps.storage, lp).unwrap();
        assert_eq!(CommitmentState::ACCEPTED, added_commitment.state);
        assert_eq!(None, added_commitment.settlment_date);

        // We need to check capital
        let paid_capital = paid_in_capital::get(&deps.storage, commitment.lp);
        for capital in &paid_capital {
            assert_eq!(0, capital.amount.u128());
        }
    }

    #[test]
    fn test_handle_succeeds_with_multiple_commits() {
        let gp = Addr::unchecked("gp");
        let lp1 = Addr::unchecked("lp1");
        let lp2 = Addr::unchecked("lp2");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        let env = mock_env();
        settlement_tester.setup_test_state(deps.as_mut().storage);

        settlement_tester.create_security_commitments(2);

        // Add these to the supported types
        let commitment1 = Commitment::new(
            lp1.clone(),
            vec![settlement_tester.security_commitments[0].clone()],
        );
        commits::set(deps.as_mut().storage, &commitment1).unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            settlement_tester.security_commitments[0].name.clone(),
            settlement_tester.security_commitments[0].amount.u128(),
        )
        .unwrap();

        let commitment2 = Commitment::new(
            lp2.clone(),
            vec![settlement_tester.security_commitments[1].clone()],
        );
        commits::set(deps.as_mut().storage, &commitment2).unwrap();
        remaining_securities::set(
            deps.as_mut().storage,
            settlement_tester.security_commitments[1].name.clone(),
            settlement_tester.security_commitments[1].amount.u128(),
        )
        .unwrap();

        let res = handle(deps.as_mut(), env, gp.clone(), vec![lp1, lp2]).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(
            Attribute::new("action", "accept_commitments"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("gp", gp), res.attributes[1]);
        assert_eq!(res.events.len(), 2);
        assert_eq!(res.events[0].attributes.len(), 1);
        assert_eq!(Attribute::new("lp", "lp1"), res.events[0].attributes[0]);
        assert_eq!(res.events[1].attributes.len(), 1);
        assert_eq!(Attribute::new("lp", "lp2"), res.events[1].attributes[0]);
    }

    #[test]
    fn test_handle_succeeds_with_no_commits() {
        let gp = Addr::unchecked("gp");
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        state::set(
            deps.as_mut().storage,
            &State::new(gp.clone(), "denom".to_string(), vec![]),
        )
        .unwrap();

        let res = handle(deps.as_mut(), env, gp.clone(), vec![]).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.events.len(), 0);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "accept_commitments");
        assert_eq!(res.attributes[1].key, "gp");
        assert_eq!(res.attributes[1].value, gp);
    }

    #[test]
    fn test_handle_must_be_triggered_by_gp() {
        let gp = Addr::unchecked("gp");
        let sender = Addr::unchecked("lp1");
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        state::set(
            deps.as_mut().storage,
            &State::new(gp, "denom".to_string(), vec![]),
        )
        .unwrap();

        let error = handle(deps.as_mut(), env, sender, vec![]).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
