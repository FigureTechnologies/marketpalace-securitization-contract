use cosmwasm_std::{Addr, Response, Storage};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::{
        commits::{self},
        paid_in_capital::PAID_IN_CAPITAL,
        state::STATE,
    },
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(deps: ProvDepsMut, sender: Addr, commitments: Vec<Addr>) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
    if sender != state.gp {
        return Err(crate::core::error::ContractError::Unauthorized {});
    }

    for lp in commitments {
        accept_commitment(deps.storage, lp)?;
    }
    Ok(Response::new())
}

fn accept_commitment(storage: &mut dyn Storage, lp: Addr) -> Result<(), ContractError> {
    let mut commitment = commits::get(storage, lp)?;

    if commitment.state != CommitmentState::PENDING {
        return Err(ContractError::InvalidCommitmentState {});
    }

    commitment.state = CommitmentState::ACCEPTED;
    commits::set(storage, &commitment)?;

    track_paid_capital(storage, commitment)?;
    Ok(())
}

fn track_paid_capital(
    storage: &mut dyn Storage,
    mut commitment: Commitment,
) -> Result<(), ContractError> {
    commitment.clear_amounts();
    PAID_IN_CAPITAL.save(storage, commitment.lp, &commitment.commitments)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::error::ContractError,
        execute::settlement::commitment::{Commitment, CommitmentState},
        storage::{
            commits::{self},
            paid_in_capital::PAID_IN_CAPITAL,
            state::{State, STATE},
        },
        util::testing::SettlementTester,
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
        assert!(false);
    }

    #[test]
    fn test_track_paid_capital_makes_an_empty_entry() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());

        track_paid_capital(deps.as_mut().storage, commitment.clone()).unwrap();
        let paid_capital = PAID_IN_CAPITAL
            .load(deps.as_mut().storage, commitment.lp)
            .unwrap();
        for capital in &paid_capital {
            assert_eq!(0, capital.amount);
        }
    }

    #[test]
    fn test_accept_commit_succeeds() {
        let lp = Addr::unchecked("address");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commits::set(deps.as_mut().storage, &commitment).unwrap();
        accept_commitment(deps.as_mut().storage, lp.clone()).unwrap();

        // We need to check the state
        let added_commitment = commits::get(&deps.storage, lp).unwrap();
        assert_eq!(CommitmentState::ACCEPTED, added_commitment.state);

        // We need to check capital
        let paid_capital = PAID_IN_CAPITAL
            .load(deps.as_mut().storage, commitment.lp)
            .unwrap();
        for capital in &paid_capital {
            assert_eq!(0, capital.amount);
        }
    }

    #[test]
    fn test_handle_succeeds_with_multiple_commits() {
        let gp = Addr::unchecked("gp");
        let lp1 = Addr::unchecked("lp1");
        let lp2 = Addr::unchecked("lp2");
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);

        settlement_tester.create_security_commitments(2);

        // Add these to the supported types
        let commitment1 = Commitment::new(
            lp1.clone(),
            vec![settlement_tester.security_commitments[0].clone()],
        );
        commits::set(deps.as_mut().storage, &commitment1).unwrap();

        let commitment2 = Commitment::new(
            lp2.clone(),
            vec![settlement_tester.security_commitments[1].clone()],
        );
        commits::set(deps.as_mut().storage, &commitment2).unwrap();

        handle(deps.as_mut(), gp, vec![lp1, lp2]).unwrap();
    }

    #[test]
    fn test_handle_succeeds_with_no_commits() {
        let gp = Addr::unchecked("gp");
        let mut deps = mock_dependencies(&[]);
        STATE
            .save(
                deps.as_mut().storage,
                &State::new(gp.clone(), "denom".to_string(), vec![]),
            )
            .unwrap();

        handle(deps.as_mut(), gp, vec![]).unwrap();
    }

    #[test]
    fn test_handle_must_be_triggered_by_gp() {
        let gp = Addr::unchecked("gp");
        let sender = Addr::unchecked("lp1");
        let mut deps = mock_dependencies(&[]);
        STATE
            .save(
                deps.as_mut().storage,
                &State::new(gp, "denom".to_string(), vec![]),
            )
            .unwrap();

        let error = handle(deps.as_mut(), sender, vec![]).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
