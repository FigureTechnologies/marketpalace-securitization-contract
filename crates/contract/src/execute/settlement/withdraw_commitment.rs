use cosmwasm_std::{Addr, Env, Event, Response, Storage};
use provwasm_std::{mint_marker_supply, transfer_marker_coins, withdraw_coins};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvMsg, ProvTxResponse},
        error::ContractError,
    },
    storage::{
        available_capital::{self},
        commits::{self},
        state::{self},
    },
    util::{self, to},
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(mut deps: ProvDepsMut, env: Env, sender: Addr, commitment: Addr) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        return Err(ContractError::Unauthorized {});
    }

    withdraw_commitment(&mut deps, &env, sender, commitment)
}

pub fn withdraw_commitment(
    deps: &mut ProvDepsMut,
    env: &Env,
    sender: Addr,
    lp: Addr,
) -> ProvTxResponse {
    let commitment = commits::get(deps.storage, lp.clone())?;

    if util::settlement::is_expired(env, &commitment) {
        return Err(ContractError::SettlmentExpired {});
    }

    if !util::settlement::is_settling(deps.storage, &commitment) {
        return Err(ContractError::CommitmentNotMet {});
    }

    let withdraw_messages = process_withdraw(deps.storage, &sender, &lp, &env.contract.address)?;

    Ok(Response::new()
        .add_messages(withdraw_messages)
        .add_event(Event::new("settled").add_attribute("lp", lp))
        .add_attribute("action", "withdraw_commitment")
        .add_attribute("gp", sender))
}

fn process_withdraw(
    storage: &mut dyn Storage,
    gp: &Addr,
    lp: &Addr,
    contract: &Addr,
) -> Result<Vec<ProvMsg>, ContractError> {
    let mut commitment = commits::get(storage, lp.clone())?;
    let capital = available_capital::remove_capital(storage, lp.clone())?;
    let mut messages = vec![];

    commitment.state = CommitmentState::SETTLED;
    messages.extend(transfer_investment_tokens(&commitment, contract)?);
    if !capital.amount.is_zero() {
        messages.push(transfer_marker_coins(
            capital.amount.u128(),
            capital.denom,
            gp.clone(),
            commitment.lp.clone(),
        )?);
    }

    commits::set(storage, &commitment)?;
    Ok(messages)
}

fn transfer_investment_tokens(
    commitment: &Commitment,
    contract: &Addr,
) -> Result<Vec<ProvMsg>, ContractError> {
    let mut messages = vec![];
    for security in &commitment.commitments {
        let investment_name = to::security_to_investment_name(&security.name, contract);
        let mint_msg = mint_marker_supply(security.amount.u128(), &investment_name)?;
        let withdraw_msg = withdraw_coins(
            &investment_name,
            security.amount.u128(),
            &investment_name,
            commitment.lp.clone(),
        )?;
        messages.push(mint_msg);
        messages.push(withdraw_msg);
    }
    Ok(messages)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Coin, Event, Uint128, Uint64};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{mint_marker_supply, withdraw_coins};

    use crate::{
        core::error::ContractError,
        execute::settlement::commitment::{Commitment, CommitmentState},
        storage::{
            available_capital::{self},
            commits::{self},
            paid_in_capital::{self},
        },
        util::{testing::SettlementTester, to},
    };

    use super::{handle, process_withdraw, transfer_investment_tokens, withdraw_commitment};

    #[test]
    fn test_transfer_investment_tokens_success() {
        let contract = Addr::unchecked("contract");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let commitment =
            Commitment::new(lp.clone(), settlement_tester.security_commitments.clone());
        let mut expected = vec![];
        for commitment in &commitment.commitments {
            let investment_name = to::security_to_investment_name(&commitment.name, &contract);
            let mint_msg = mint_marker_supply(commitment.amount.u128(), &investment_name).unwrap();
            let withdraw_msg = withdraw_coins(
                &investment_name,
                commitment.amount.u128(),
                &investment_name,
                lp.clone(),
            )
            .unwrap();
            expected.push(mint_msg);
            expected.push(withdraw_msg);
        }

        let transferred = transfer_investment_tokens(&commitment, &contract).unwrap();
        assert_eq!(transferred.len(), 4);
        assert_eq!(expected, transferred);
    }

    #[test]
    fn test_process_withdraw_fails_when_commit_doesnt_exist() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let contract = Addr::unchecked("contract");
        process_withdraw(deps.as_mut().storage, &gp, &lp, &contract).unwrap_err();
    }

    #[test]
    fn test_process_withdraw_fails_when_available_capital_doesnt_exist() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let contract = Addr::unchecked("contract");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        process_withdraw(deps.as_mut().storage, &gp, &commitment.lp, &contract).unwrap_err();
    }

    #[test]
    fn test_process_withdraw_has_capital() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let contract = Addr::unchecked("contract");
        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(100, "denom".to_string())],
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments,
        )
        .unwrap();
        let messages =
            process_withdraw(deps.as_mut().storage, &gp, &commitment.lp, &contract).unwrap();

        let updated = commits::get(&deps.storage, commitment.lp.clone()).unwrap();
        assert_eq!(CommitmentState::SETTLED, updated.state);
        assert_eq!(5, messages.len());
        assert_eq!(
            false,
            available_capital::has_lp(deps.as_mut().storage, commitment.lp)
        );
    }

    #[test]
    fn test_process_withdraw_has_no_capital() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let gp = Addr::unchecked("gp");
        let contract = Addr::unchecked("contract");
        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(0, "denom".to_string())],
        )
        .unwrap();

        paid_in_capital::set(
            deps.as_mut().storage,
            commitment.lp.clone(),
            &settlement_tester.security_commitments,
        )
        .unwrap();
        let messages =
            process_withdraw(deps.as_mut().storage, &gp, &commitment.lp, &contract).unwrap();

        let updated = commits::get(&deps.storage, commitment.lp.clone()).unwrap();
        assert_eq!(CommitmentState::SETTLED, updated.state);
        assert_eq!(4, messages.len());
        assert_eq!(
            false,
            available_capital::has_lp(deps.as_mut().storage, commitment.lp)
        );
    }

    #[test]
    fn test_withdraw_commitments_with_invalid_lp() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("gp");
        let lp = Addr::unchecked("lp");
        withdraw_commitment(&mut deps.as_mut(), &mock_env(), sender, lp).unwrap_err();
    }

    #[test]
    fn test_withdraw_commitments_with_not_settled() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let sender = Addr::unchecked("gp");
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

        let mut partial_paid = settlement_tester.security_commitments.clone();
        partial_paid[0].amount = Uint128::new(1);
        paid_in_capital::set(deps.as_mut().storage, commitment.lp.clone(), &partial_paid).unwrap();

        let err = withdraw_commitment(
            &mut deps.as_mut(),
            &mock_env(),
            sender.clone(),
            commitment.lp.clone(),
        )
        .unwrap_err();
        assert_eq!(
            ContractError::CommitmentNotMet {}.to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_withdraw_commitments_with_expired_settlement() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let sender = Addr::unchecked("gp");
        let capital_denom = "denom".to_string();
        let mut commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());
        commitment.state = CommitmentState::ACCEPTED;
        commitment.settlment_date = Some(Uint64::new(mock_env().block.time.seconds() - 1));

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        available_capital::add_capital(
            deps.as_mut().storage,
            commitment.lp.clone(),
            vec![Coin::new(100, &capital_denom)],
        )
        .unwrap();

        let mut partial_paid = settlement_tester.security_commitments.clone();
        partial_paid[0].amount = Uint128::new(1);
        paid_in_capital::set(deps.as_mut().storage, commitment.lp.clone(), &partial_paid).unwrap();

        let err = withdraw_commitment(
            &mut deps.as_mut(),
            &mock_env(),
            sender.clone(),
            commitment.lp.clone(),
        )
        .unwrap_err();
        assert_eq!(
            ContractError::SettlmentExpired {}.to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_withdraw_commitments_with_settled() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
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

        let res = withdraw_commitment(
            &mut deps.as_mut(),
            &mock_env(),
            gp.clone(),
            commitment.lp.clone(),
        )
        .unwrap();
        assert_eq!(5, res.messages.len());
        assert_eq!(1, res.events.len());
        assert_eq!(
            Event::new("settled").add_attribute("lp", commitment.lp),
            res.events[0]
        );
    }

    #[test]
    fn test_handle_must_be_gp() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");

        let settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);

        let error = handle(deps.as_mut(), mock_env(), sender.clone(), sender).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_handle_succeeds() {
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

        let res = handle(deps.as_mut(), mock_env(), gp.clone(), commitment.lp.clone()).unwrap();
        assert_eq!(2, res.attributes.len());
        assert_eq!(1, res.events.len());
        assert_eq!(
            Attribute::new("action", "withdraw_commitment"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("gp", gp), res.attributes[1]);
    }
}
