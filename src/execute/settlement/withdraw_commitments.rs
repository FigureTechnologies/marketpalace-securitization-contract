use cosmwasm_std::{Addr, BankMsg, Coin, Env, Response, StdResult, Storage, Uint128};
use provwasm_std::{mint_marker_supply, withdraw_coins};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvMsg, ProvTxResponse},
        error::ContractError,
        state::{AVAILABLE_CAPITAL, COMMITS, PAID_IN_CAPITAL, STATE},
    },
    util::to,
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
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
    let keys: StdResult<Vec<_>> = AVAILABLE_CAPITAL
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    let lps = keys.unwrap();

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
    let mut commitment = COMMITS.load(storage, lp.clone())?;
    let amount = remove_deposited_capital(storage, lp)?;
    let mut messages = vec![];

    if is_settling(storage, &commitment)? {
        commitment.state = CommitmentState::SETTLED;

        messages.extend(transfer_investment_tokens(&commitment, contract)?);
    }

    COMMITS.save(storage, lp.clone(), &commitment)?;
    Ok((messages, amount))
}

fn transfer_investment_tokens(
    commitment: &Commitment,
    contract: &Addr,
) -> Result<Vec<ProvMsg>, ContractError> {
    let mut messages = vec![];
    for security in &commitment.commitments {
        let investment_name = to::security_to_investment_name(&security.name, contract);
        let mint_msg = mint_marker_supply(security.amount, &investment_name)?;
        let withdraw_msg = withdraw_coins(
            &investment_name,
            security.amount,
            &investment_name,
            commitment.lp.clone(),
        )?;
        messages.push(mint_msg);
        messages.push(withdraw_msg);
    }
    Ok(messages)
}

fn remove_deposited_capital(
    storage: &mut dyn Storage,
    key: &Addr,
) -> Result<Uint128, ContractError> {
    let capital = AVAILABLE_CAPITAL.load(storage, key.clone())?;
    AVAILABLE_CAPITAL.remove(storage, key.clone());
    Ok(capital[0].amount)
}

fn is_settling(storage: &dyn Storage, commitment: &Commitment) -> Result<bool, ContractError> {
    let paid_in_capital = PAID_IN_CAPITAL.load(storage, commitment.lp.clone())?;
    Ok(paid_in_capital == commitment.commitments && commitment.state == CommitmentState::ACCEPTED)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Coin, Uint128};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{mint_marker_supply, withdraw_coins};

    use crate::{
        core::{
            error::ContractError,
            security::SecurityCommitment,
            state::{State, AVAILABLE_CAPITAL, COMMITS, PAID_IN_CAPITAL, STATE},
        },
        execute::settlement::commitment::{Commitment, CommitmentState},
        util::{self, to},
    };

    use super::{
        handle, is_settling, process_withdraw, remove_deposited_capital,
        transfer_investment_tokens, withdraw_commitments,
    };

    #[test]
    fn test_is_settling_success() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let security_commitment: Vec<SecurityCommitment> = vec![SecurityCommitment {
            name: "Security 1".to_string(),
            amount: 50,
        }];
        let mut commitment = Commitment::new(lp.clone(), security_commitment.clone());
        commitment.state = CommitmentState::ACCEPTED;
        PAID_IN_CAPITAL
            .save(deps.as_mut().storage, lp.clone(), &security_commitment)
            .unwrap();
        let settling = is_settling(&deps.storage, &commitment).unwrap();
        assert_eq!(true, settling);
    }

    #[test]
    fn test_is_settling_fails_on_already_settled() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let security_commitment: Vec<SecurityCommitment> = vec![];
        let mut commitment = Commitment::new(lp.clone(), security_commitment.clone());
        commitment.state = CommitmentState::SETTLED;
        PAID_IN_CAPITAL
            .save(deps.as_mut().storage, lp.clone(), &security_commitment)
            .unwrap();
        let settling = is_settling(&deps.storage, &commitment).unwrap();
        assert_eq!(false, settling);
    }

    #[test]
    fn test_is_settling_handles_invalid_lp() {
        let deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let security_commitment: Vec<SecurityCommitment> = vec![];
        let commitment = Commitment::new(lp.clone(), security_commitment);
        is_settling(&deps.storage, &commitment).unwrap_err();
    }

    #[test]
    fn test_is_settling_fails_on_missing_capital() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let security_commitment: Vec<SecurityCommitment> = vec![SecurityCommitment {
            name: "Security 1".to_string(),
            amount: 50,
        }];
        let mut commitment = Commitment::new(lp.clone(), security_commitment.clone());
        commitment.state = CommitmentState::ACCEPTED;
        let mut capital = commitment.clone();
        capital.clear_amounts();
        PAID_IN_CAPITAL
            .save(deps.as_mut().storage, lp.clone(), &capital.commitments)
            .unwrap();
        let settling = is_settling(&deps.storage, &commitment).unwrap();
        assert_eq!(false, settling);
    }

    #[test]
    fn test_remove_deposit_capital_success() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let funds = vec![Coin::new(50, "denom".to_string())];
        AVAILABLE_CAPITAL
            .save(deps.as_mut().storage, lp.clone(), &funds)
            .unwrap();

        let removed = remove_deposited_capital(deps.as_mut().storage, &lp).unwrap();
        assert_eq!(Uint128::new(50), removed);
        AVAILABLE_CAPITAL
            .load(deps.as_mut().storage, lp.clone())
            .unwrap_err();
    }

    #[test]
    fn test_remove_deposit_handles_invalid_lp() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        remove_deposited_capital(deps.as_mut().storage, &lp).unwrap_err();
    }

    #[test]
    fn test_transfer_investment_tokens_success() {
        let contract = Addr::unchecked("contract");
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(
            lp.clone(),
            vec![
                SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: 5,
                },
                SecurityCommitment {
                    name: "Security2".to_string(),
                    amount: 7,
                },
            ],
        );
        let mut expected = vec![];
        for commitment in &commitment.commitments {
            let investment_name = to::security_to_investment_name(&commitment.name, &contract);
            let mint_msg = mint_marker_supply(commitment.amount, &investment_name).unwrap();
            let withdraw_msg = withdraw_coins(
                &investment_name,
                commitment.amount,
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
        let lp = Addr::unchecked("lp");
        let contract = Addr::unchecked("contract");
        let commitment = Commitment::new(
            lp,
            vec![
                SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: 5,
                },
                SecurityCommitment {
                    name: "Security2".to_string(),
                    amount: 7,
                },
            ],
        );

        COMMITS
            .save(deps.as_mut().storage, commitment.lp.clone(), &commitment)
            .unwrap();

        process_withdraw(deps.as_mut().storage, &commitment.lp, &contract).unwrap_err();
    }

    #[test]
    fn test_process_withdraw_not_settled() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let contract = Addr::unchecked("contract");
        let mut commitment = Commitment::new(
            lp,
            vec![
                SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: 5,
                },
                SecurityCommitment {
                    name: "Security2".to_string(),
                    amount: 7,
                },
            ],
        );
        commitment.state = CommitmentState::ACCEPTED;

        COMMITS
            .save(deps.as_mut().storage, commitment.lp.clone(), &commitment)
            .unwrap();

        AVAILABLE_CAPITAL
            .save(
                deps.as_mut().storage,
                commitment.lp.clone(),
                &vec![Coin::new(100, "denom".to_string())],
            )
            .unwrap();

        PAID_IN_CAPITAL
            .save(
                deps.as_mut().storage,
                commitment.lp.clone(),
                &vec![
                    SecurityCommitment {
                        name: "Security1".to_string(),
                        amount: 1,
                    },
                    SecurityCommitment {
                        name: "Security2".to_string(),
                        amount: 1,
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
            AVAILABLE_CAPITAL.has(deps.as_mut().storage, commitment.lp)
        );
    }

    #[test]
    fn test_process_withdraw_settled() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let contract = Addr::unchecked("contract");
        let mut commitment = Commitment::new(
            lp,
            vec![
                SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: 5,
                },
                SecurityCommitment {
                    name: "Security2".to_string(),
                    amount: 7,
                },
            ],
        );
        commitment.state = CommitmentState::ACCEPTED;

        COMMITS
            .save(deps.as_mut().storage, commitment.lp.clone(), &commitment)
            .unwrap();

        AVAILABLE_CAPITAL
            .save(
                deps.as_mut().storage,
                commitment.lp.clone(),
                &vec![Coin::new(100, "denom".to_string())],
            )
            .unwrap();

        PAID_IN_CAPITAL
            .save(
                deps.as_mut().storage,
                commitment.lp.clone(),
                &vec![
                    SecurityCommitment {
                        name: "Security1".to_string(),
                        amount: 5,
                    },
                    SecurityCommitment {
                        name: "Security2".to_string(),
                        amount: 7,
                    },
                ],
            )
            .unwrap();

        let (messages, amount) =
            process_withdraw(deps.as_mut().storage, &commitment.lp, &contract).unwrap();

        let updated = COMMITS
            .load(deps.as_mut().storage, commitment.lp.clone())
            .unwrap();
        assert_eq!(CommitmentState::SETTLED, updated.state);
        assert_eq!(4, messages.len());
        assert_eq!(Uint128::new(100), amount);
        assert_eq!(
            false,
            AVAILABLE_CAPITAL.has(deps.as_mut().storage, commitment.lp)
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
        let lp = Addr::unchecked("lp");
        let capital_denom = "denom".to_string();
        let mut commitment = Commitment::new(
            lp,
            vec![
                SecurityCommitment {
                    name: "Security1".to_string(),
                    amount: 5,
                },
                SecurityCommitment {
                    name: "Security2".to_string(),
                    amount: 7,
                },
            ],
        );
        commitment.state = CommitmentState::ACCEPTED;

        COMMITS
            .save(deps.as_mut().storage, commitment.lp.clone(), &commitment)
            .unwrap();

        AVAILABLE_CAPITAL
            .save(
                deps.as_mut().storage,
                commitment.lp.clone(),
                &vec![Coin::new(100, &capital_denom)],
            )
            .unwrap();

        PAID_IN_CAPITAL
            .save(
                deps.as_mut().storage,
                commitment.lp.clone(),
                &vec![
                    SecurityCommitment {
                        name: "Security1".to_string(),
                        amount: 5,
                    },
                    SecurityCommitment {
                        name: "Security2".to_string(),
                        amount: 7,
                    },
                ],
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

        util::testing::setup_test_state(deps.as_mut().storage);

        let error = handle(deps.as_mut(), mock_env(), sender).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
