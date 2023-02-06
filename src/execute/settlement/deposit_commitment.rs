use cosmwasm_std::{Addr, Coin, Order, Response, StdResult, Uint128};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    security::SecurityCommitment,
    state::{AVAILABLE_CAPITAL, COMMITS, PAID_IN_CAPITAL, SECURITIES_MAP, STATE},
};

use super::commitment::CommitmentState;

pub fn handle(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    deposit: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let commitment = COMMITS.load(deps.storage, sender.clone())?;
    let state = STATE.load(deps.storage)?;
    if commitment.state != CommitmentState::ACCEPTED {
        return Err(crate::core::error::ContractError::InvalidCommitmentState {});
    }

    if !drawdown_met(&deps, &deposit) {
        return Err(crate::core::error::ContractError::InvalidSecurityCommitment {});
    }

    // Validate that we have the funds
    let expected_funds = calculate_funds(&deps, &deposit, &state.capital_denom);
    let has_funds = expected_funds.iter().all(|coin| funds.contains(coin));
    if expected_funds.len() != funds.len() || !has_funds {
        return Err(crate::core::error::ContractError::FundMismatch {});
    }

    Ok(update_capital(deps, sender, funds, deposit)?)
}

// The purpose of this function is to add new_commitment to commitments.
// We do this by finding the security commitment that has the same name as new_commitment,
// and then we add the new_commitment.amount to the commitment.amount.
//
// Note this modifies commitments
fn add_security_commitment(
    new_commitment: &SecurityCommitment,
    commitments: &mut Vec<SecurityCommitment>,
) {
    for commitment in commitments.iter_mut() {
        if commitment.name == new_commitment.name {
            commitment.amount += new_commitment.amount;
        }
    }
}

// The purpose of this function is to add a coin to capital.
// We do this by finding the coin that has the same name as new_coin,
// and then we add the new_coin.amount to the coin.amount.
//
// Note this modifies capital
fn add_to_capital(new_coin: &Coin, capital: &mut Vec<Coin>) {
    for coin in capital.iter_mut() {
        if coin.denom == new_coin.denom {
            coin.amount += new_coin.amount;
        }
    }
}

// This updates the AVAILABLE_CAPITAL and the PAID_IN_CAPITAL
fn update_capital(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    deposit: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    PAID_IN_CAPITAL.update(
        deps.storage,
        sender.clone(),
        |already_committed| -> StdResult<Vec<SecurityCommitment>> {
            match already_committed {
                None => Ok(deposit),
                Some(mut already_committed) => {
                    for deposit_security in &deposit {
                        add_security_commitment(deposit_security, &mut already_committed);
                    }
                    Ok(already_committed)
                }
            }
        },
    )?;

    AVAILABLE_CAPITAL.update(
        deps.storage,
        sender.clone(),
        |available_capital| -> StdResult<Vec<Coin>> {
            match available_capital {
                None => Ok(funds),
                Some(mut available_capital) => {
                    for fund_coin in &funds {
                        add_to_capital(fund_coin, &mut available_capital);
                    }
                    Ok(available_capital)
                }
            }
        },
    )?;
    Ok(Response::default())
}

// The purpose of this function is to make sure we have a valid drawdown.
// Check that the length is the same between the initial drawdown and our instatiation
// We check to make sure that every security commitment in the drawdown was specified at instantiation
// We also make sure our initial drawdown has the minimum for each of these security commitments
fn drawdown_met(deps: &ProvDepsMut, initial_drawdown: &Vec<SecurityCommitment>) -> bool {
    let security_types: StdResult<Vec<_>> = SECURITIES_MAP
        .keys(deps.storage, None, None, Order::Ascending)
        .collect();
    let security_types = security_types.unwrap();

    if security_types.len() != initial_drawdown.len() {
        return false;
    }

    for drawdown in initial_drawdown {
        let security = SECURITIES_MAP.load(deps.storage, drawdown.name.clone());
        if security.is_err() {
            return false;
        }
        let security = security.unwrap();
        if drawdown.amount < security.minimum_amount {
            return false;
        }
    }

    true
}

// We are strict that all capital must be in the same denom
fn calculate_funds(
    deps: &ProvDepsMut,
    initial_drawdown: &[SecurityCommitment],
    capital_denom: &String,
) -> Vec<Coin> {
    let mut sum = Coin::new(0, capital_denom);

    for security_commitment in initial_drawdown {
        let security = SECURITIES_MAP
            .load(deps.storage, security_commitment.name.clone())
            .unwrap();

        let cost = Uint128::from(security_commitment.amount) * security.price_per_unit.amount;
        sum.amount += cost;
    }

    vec![sum]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_funds_empty_initial_drawdown() {
        assert!(false);
    }

    #[test]
    fn test_calculate_funds_is_successful() {
        assert!(false);
    }

    #[test]
    fn test_calculate_funds_invalid_security() {
        assert!(false);
    }

    #[test]
    fn test_drawdown_met_with_not_all_securities() {
        assert!(false);
    }

    #[test]
    fn test_drawdown_met_with_invalid_security() {
        assert!(false);
    }

    #[test]
    fn test_drawdown_met_with_amount_less_than_minimum() {
        assert!(false);
    }

    #[test]
    fn test_drawdown_met_is_successful() {
        assert!(false);
    }

    #[test]
    fn test_handle_throws_error_with_invalid_commitment_state() {
        assert!(false);
    }

    #[test]
    fn test_handle_throws_error_when_initial_drawdown_not_met() {
        assert!(false);
    }

    #[test]
    fn test_handle_throws_error_when_not_enough_funds() {
        assert!(false);
    }

    #[test]
    fn test_handle_is_successful() {
        assert!(false);
    }
}
