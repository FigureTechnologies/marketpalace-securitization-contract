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

    gp_withdraw(deps, env, sender, state.capital_denom)
}

fn gp_withdraw(deps: ProvDepsMut, env: Env, sender: Addr, capital_denom: String) -> ProvTxResponse {
    let mut messages = vec![];
    let mut response = Response::new();
    let keys: StdResult<Vec<_>> = AVAILABLE_CAPITAL
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    let keys = keys.unwrap();

    let mut send_amount = Coin::new(0, capital_denom);
    for key in keys {
        let mut commitment = COMMITS.load(deps.storage, key.clone())?;
        send_amount.amount += remove_deposited_capital(deps.storage, &key)?;

        if is_settling(&deps, &key, &commitment)? {
            commitment.state = CommitmentState::SETTLED;

            messages.extend(transfer_investment_tokens(
                &commitment,
                &key,
                &env.contract.address,
            )?);
        }

        COMMITS.save(deps.storage, key.clone(), &commitment)?;
    }

    if !send_amount.amount.is_zero() {
        response = response.add_message(BankMsg::Send {
            to_address: sender.to_string(),
            amount: vec![send_amount],
        });
    }
    Ok(response.add_messages(messages))
}

fn is_settling(
    deps: &ProvDepsMut,
    key: &Addr,
    commitment: &Commitment,
) -> Result<bool, ContractError> {
    let paid_in_capital = PAID_IN_CAPITAL.load(deps.storage, key.clone())?;
    Ok(paid_in_capital == commitment.commitments && commitment.state == CommitmentState::ACCEPTED)
}

fn remove_deposited_capital(
    storage: &mut dyn Storage,
    key: &Addr,
) -> Result<Uint128, ContractError> {
    let capital = AVAILABLE_CAPITAL.load(storage, key.clone())?;
    AVAILABLE_CAPITAL.remove(storage, key.clone());
    Ok(capital[0].amount)
}

fn transfer_investment_tokens(
    commitment: &Commitment,
    recipient: &Addr,
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
            recipient.clone(),
        )?;
        messages.push(mint_msg);
        messages.push(withdraw_msg);
    }
    Ok(messages)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_funds_are_empty() {
        assert!(false);
    }

    #[test]
    fn test_accepted_commits_change_to_settled() {
        assert!(false);
    }

    #[test]
    fn test_pending_commits_throw_error() {
        assert!(false);
    }

    #[test]
    fn test_invested_commits_are_not_changed_to_settled() {
        assert!(false);
    }

    #[test]
    fn test_available_capital_is_cleared() {
        assert!(false);
    }

    #[test]
    fn test_completed_commits_are_changed_to_invested_and_reward() {
        assert!(false);
    }

    #[test]
    fn test_correct_amount_is_withdrawn() {
        assert!(false);
    }
}
