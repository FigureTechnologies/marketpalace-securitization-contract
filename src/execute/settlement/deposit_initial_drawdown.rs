use cosmwasm_std::{Addr, Coin, Order, Response, StdResult, Uint128};

use crate::{
    commitment::CommitmentState,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        security::SecurityCommitment,
        state::{AVAILABLE_CAPITAL, COMMITS, PAID_IN_CAPITAL, SECURITIES_MAP, STATE},
    },
};

pub fn handle(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    initial_drawdown: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let commitment = COMMITS.load(deps.storage, sender.clone())?;
    let state = STATE.load(deps.storage)?;
    if commitment.state != CommitmentState::ACCEPTED {
        // TODO
        // Throw an error
    }

    if !drawdown_met(&deps, &initial_drawdown) {
        // TODO
        // Throw an error
    }

    // Check that the correct of funds passed in exactly match expected
    let expected_funds = calculate_funds(&deps, &initial_drawdown, &state.capital_denom);
    let has_funds = expected_funds.iter().all(|coin| funds.contains(coin));
    if expected_funds.len() != funds.len() || !has_funds {
        // TODO
        // Throw an error
    }

    // Update the paid in capital with the initial drawdown
    PAID_IN_CAPITAL.save(deps.storage, sender.clone(), &initial_drawdown)?;

    // Update the capital for the gp.
    AVAILABLE_CAPITAL.save(deps.storage, sender, &funds)?;

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
