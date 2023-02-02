use cosmwasm_std::{Addr, BankMsg, Coin, Env, Response, StdResult};
use provwasm_std::{mint_marker_supply, withdraw_coins};

use crate::{
    commitment::CommitmentState,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        state::{AVAILABLE_CAPITAL, COMMITS, PAID_IN_CAPITAL, STATE},
    },
    util::to,
};

pub fn handle(deps: ProvDepsMut, env: Env, sender: Addr) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
    if sender == state.gp {
        gp_withdraw(deps, env, sender, state.capital_denom)
    } else {
        lp_withdraw(deps, env, sender, state.capital_denom)
    }
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
        let capital = AVAILABLE_CAPITAL.load(deps.storage, key.clone())?;

        // Update commitment as settled if it's in accepted.
        let mut commitment = COMMITS.load(deps.storage, key.clone())?;

        send_amount.amount += capital[0].amount;

        AVAILABLE_CAPITAL.remove(deps.storage, key.clone());

        let paid_in_capital = PAID_IN_CAPITAL.load(deps.storage, key.clone())?;
        if paid_in_capital == commitment.commitments
            && commitment.state == CommitmentState::ACCEPTED
        {
            commitment.state = CommitmentState::SETTLED;

            // We can now mint the investment token and send it to them
            for security in &commitment.commitments {
                let investment_name =
                    to::security_to_investment_name(&security.name, &env.contract.address);
                let mint_msg = mint_marker_supply(security.amount, &investment_name)?;
                let withdraw_msg = withdraw_coins(
                    &investment_name,
                    security.amount,
                    &investment_name,
                    key.clone(),
                )?;
                messages.push(mint_msg);
                messages.push(withdraw_msg);
            }
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

fn lp_withdraw(
    _deps: ProvDepsMut,
    _env: Env,
    _sender: Addr,
    _capital_denom: String,
) -> ProvTxResponse {
    Ok(Response::default())
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
