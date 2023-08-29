use cosmwasm_std::{Env, Storage, Timestamp};

use crate::{
    core::error::ContractError,
    execute::settlement::commitment::{Commitment, CommitmentState},
    storage::{self, paid_in_capital},
};

/// Checks if the defined settlment_date within the commitment has expired. A settlement_date
/// of None will never expire.
///
/// # Parameters
///
/// * `env` A reference to an environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `commitment` A reference to the commitment to check the settlement_date on.
///
/// # Examples

pub fn is_expired(env: &Env, commitment: &Commitment) -> bool {
    if let Some(settlement_time) = commitment.settlment_date {
        return env.block.time.seconds() > settlement_time.u64();
    }
    false
}

/// Checks if the timestamp is expired.
///
/// # Parameters
///
/// * `storage` A reference to the storage object in the dependencies provided by the cosmwasm framework.
/// * `time` A reference to the timestamp that will be checked for expiration.
///
/// # Examples

pub fn timestamp_is_expired(
    storage: &dyn Storage,
    time: &Timestamp,
) -> Result<bool, ContractError> {
    if let Some(settlement_time) = storage::state::get_settlement_time(storage)? {
        return Ok(time.seconds() > settlement_time.u64());
    }
    Ok(false)
}

/// Checks if the commitment can settle. A settled commitment is one that has been accepted, and the paid_in_capital
/// matches the commitment.
///
/// # Parameters
///
/// * `storage` A reference to the storage object in the dependencies provided by the cosmwasm framework.
/// * `commitment` A reference to the commitment to check.
///
/// # Examples

pub fn is_settling(storage: &dyn Storage, commitment: &Commitment) -> bool {
    let paid_in_capital = paid_in_capital::get(storage, commitment.lp.clone());
    paid_in_capital == commitment.commitments && commitment.state == CommitmentState::ACCEPTED
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Uint64};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        execute::settlement::commitment::{Commitment, CommitmentState},
        storage::paid_in_capital,
        util::{
            settlement::{is_expired, is_settling, timestamp_is_expired},
            testing::{create_test_state, SettlementTester},
        },
    };

    #[test]
    fn test_settlement_expired_with_no_settlement_date() {
        let env = mock_env();
        let commitment = Commitment::new(Addr::unchecked("lp"), vec![]);
        let res = is_expired(&env, &commitment);
        assert_eq!(false, res);
    }

    #[test]
    fn test_settlement_expired_with_future_time() {
        let env = mock_env();
        let mut commitment = Commitment::new(Addr::unchecked("lp"), vec![]);
        commitment.settlment_date = Some(Uint64::new(env.block.time.seconds()));
        let res = is_expired(&env, &commitment);
        assert_eq!(false, res);
    }

    #[test]
    fn test_settlement_expired_with_past_time() {
        let env = mock_env();
        let mut commitment = Commitment::new(Addr::unchecked("lp"), vec![]);
        commitment.settlment_date = Some(Uint64::new(env.block.time.seconds() - 1));
        let res = is_expired(&env, &commitment);
        assert_eq!(true, res);
    }

    #[test]
    fn test_timestamp_expired_with_no_settlement_date() {
        let env = mock_env();
        let mut deps = mock_dependencies(&[]);
        create_test_state(&mut deps, &env, false);
        let res = timestamp_is_expired(&deps.storage, &env.block.time).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_timestamp_expired_with_unexpired_time() {
        let env = mock_env();
        let mut deps = mock_dependencies(&[]);
        create_test_state(&mut deps, &env, true);
        let res = timestamp_is_expired(&deps.storage, &env.block.time.plus_seconds(86400)).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_timestamp_expired_with_expired_time() {
        let env = mock_env();
        let mut deps = mock_dependencies(&[]);
        create_test_state(&mut deps, &env, true);
        let res = timestamp_is_expired(&deps.storage, &env.block.time.plus_seconds(86401)).unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_is_settling_success() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("bad address");
        let mut commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;
        paid_in_capital::set(
            deps.as_mut().storage,
            lp.clone(),
            &settlement_tester.security_commitments,
        )
        .unwrap();
        let settling = is_settling(&deps.storage, &commitment);
        assert_eq!(true, settling);
    }

    #[test]
    fn test_is_settling_fails_on_already_settled() {
        let mut deps = mock_dependencies(&[]);
        let settlement_tester = SettlementTester::new();
        let lp = Addr::unchecked("bad address");
        let mut commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::SETTLED;
        paid_in_capital::set(
            deps.as_mut().storage,
            lp.clone(),
            &settlement_tester.security_commitments,
        )
        .unwrap();
        let settling = is_settling(&deps.storage, &commitment);
        assert_eq!(false, settling);
    }

    #[test]
    fn test_is_settling_handles_invalid_lp() {
        let deps = mock_dependencies(&[]);
        let settlement_tester = SettlementTester::new();
        let lp = Addr::unchecked("bad address");
        let commitment = Commitment::new(lp.clone(), settlement_tester.security_commitments);
        let settling = is_settling(&deps.storage, &commitment);
        assert_eq!(false, settling);
    }

    #[test]
    fn test_is_settling_fails_on_missing_capital() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let lp = Addr::unchecked("bad address");
        let mut commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;
        let mut capital = commitment.clone();
        capital.clear_amounts();
        paid_in_capital::set(deps.as_mut().storage, lp.clone(), &capital.commitments).unwrap();
        let settling = is_settling(&deps.storage, &commitment);
        assert_eq!(false, settling);
    }
}
