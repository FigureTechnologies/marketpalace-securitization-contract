use cosmwasm_std::{Addr, BankMsg, Coin, Env, Response, Storage, Uint128};
use provwasm_std::{mint_marker_supply, withdraw_coins};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvMsg, ProvTxResponse},
        error::ContractError,
    },
    storage::{
        available_capital::{self},
        commits::{self},
        paid_in_capital::{self},
        state::{self},
    },
    util::to,
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        return Err(ContractError::Unauthorized {});
    }

    withdraw_commitments(deps, env, sender, state.capital_denom)
}

fn withdraw_commitments(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    capital_denom: String,
) -> ProvTxResponse {
    let mut messages: Vec<ProvMsg> = vec![];
    let mut response = Response::new();
    let lps = available_capital::get_lps(deps.storage)?;

    let mut send_amount = Coin::new(0, capital_denom);
    for lp in &lps {
        let withdraw = process_withdraw(deps.storage, lp, &env.contract.address)?;
        messages.extend(withdraw.0);
        send_amount.amount += withdraw.1;
    }

    if !send_amount.amount.is_zero() {
        response = response.add_message(BankMsg::Send {
            to_address: sender.to_string(),
            amount: vec![send_amount],
        });
    }
    Ok(response.add_messages(messages))
}

fn process_withdraw(
    storage: &mut dyn Storage,
    lp: &Addr,
    contract: &Addr,
) -> Result<(Vec<ProvMsg>, Uint128), ContractError> {
    let mut commitment = commits::get(storage, lp.clone())?;
    let capital = available_capital::remove_capital(storage, lp.clone())?;
    let mut messages = vec![];

    if is_settling(storage, &commitment) {
        commitment.state = CommitmentState::SETTLED;

        messages.extend(transfer_investment_tokens(&commitment, contract)?);
    }

    commits::set(storage, &commitment)?;
    Ok((messages, capital.amount))
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

fn is_settling(storage: &dyn Storage, commitment: &Commitment) -> bool {
    let paid_in_capital = paid_in_capital::get(storage, commitment.lp.clone());
    paid_in_capital == commitment.commitments && commitment.state == CommitmentState::ACCEPTED
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Coin, Uint128};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{mint_marker_supply, withdraw_coins};

    use crate::{
        core::{error::ContractError, security::SecurityCommitment},
        execute::settlement::commitment::{Commitment, CommitmentState},
        storage::{
            available_capital::{self},
            commits::{self},
            paid_in_capital::{self},
        },
        util::{testing::SettlementTester, to},
    };

    use super::{
        handle, is_settling, process_withdraw, transfer_investment_tokens, withdraw_commitments,
    };

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
        let contract = Addr::unchecked("contract");
        process_withdraw(deps.as_mut().storage, &lp, &contract).unwrap_err();
    }

    #[test]
    fn test_process_withdraw_fails_when_available_capital_doesnt_exist() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
        let contract = Addr::unchecked("contract");
        let commitment = Commitment::new(lp, settlement_tester.security_commitments.clone());

        commits::set(deps.as_mut().storage, &commitment).unwrap();

        process_withdraw(deps.as_mut().storage, &commitment.lp, &contract).unwrap_err();
    }

    #[test]
    fn test_process_withdraw_not_settled() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
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
            &vec![
                SecurityCommitment {
                    name: settlement_tester.security_commitments[0].name.clone(),
                    amount: Uint128::new(1),
                },
                SecurityCommitment {
                    name: settlement_tester.security_commitments[1].name.clone(),
                    amount: Uint128::new(1),
                },
            ],
        )
        .unwrap();

        let (messages, amount) =
            process_withdraw(deps.as_mut().storage, &commitment.lp, &contract).unwrap();
        assert_eq!(0, messages.len());
        assert_eq!(Uint128::new(100), amount);
        assert_eq!(
            false,
            available_capital::has_lp(deps.as_mut().storage, commitment.lp)
        );
    }

    #[test]
    fn test_process_withdraw_settled() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
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
        let (messages, amount) =
            process_withdraw(deps.as_mut().storage, &commitment.lp, &contract).unwrap();

        let updated = commits::get(&deps.storage, commitment.lp.clone()).unwrap();
        assert_eq!(CommitmentState::SETTLED, updated.state);
        assert_eq!(4, messages.len());
        assert_eq!(Uint128::new(100), amount);
        assert_eq!(
            false,
            available_capital::has_lp(deps.as_mut().storage, commitment.lp)
        );
    }

    #[test]
    fn test_withdraw_commitments_with_none() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("gp");
        let capital_denom = "denom".to_string();
        let res = withdraw_commitments(deps.as_mut(), mock_env(), sender, capital_denom).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_withdraw_commitments_with_settled() {
        let mut deps = mock_dependencies(&[]);
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(2);
        let lp = Addr::unchecked("lp");
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

        let res = withdraw_commitments(
            deps.as_mut(),
            mock_env(),
            commitment.lp.clone(),
            capital_denom,
        )
        .unwrap();
        assert_eq!(5, res.messages.len());
    }

    #[test]
    fn test_handle_must_be_gp() {
        let mut deps = mock_dependencies(&[]);
        let sender = Addr::unchecked("lp");

        let settlement_tester = SettlementTester::new();
        settlement_tester.setup_test_state(deps.as_mut().storage);

        let error = handle(deps.as_mut(), mock_env(), sender).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
