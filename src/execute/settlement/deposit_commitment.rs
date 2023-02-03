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

    // Check that the correct of funds passed in exactly match expected
    let expected_funds = calculate_funds(&deps, &deposit, &state.capital_denom);
    let has_funds = expected_funds.iter().all(|coin| funds.contains(coin));
    if expected_funds.len() != funds.len() || !has_funds {
        return Err(crate::core::error::ContractError::FundMismatch {});
    }

    // If there is no entry for PAID_IN_CAPITAL then the commitment goes in as the entry
    // If there is an entry then we update the entry by adding each type of deposit security to the appropriate paid security
    PAID_IN_CAPITAL.update(
        deps.storage,
        sender.clone(),
        |already_committed| -> StdResult<Vec<SecurityCommitment>> {
            match already_committed {
                None => Ok(deposit),
                Some(mut already_committed) => {
                    for deposit_security in deposit {
                        already_committed = already_committed
                            .into_iter()
                            .map(|mut commitment_security| {
                                if commitment_security.name == deposit_security.name {
                                    commitment_security.amount += deposit_security.amount;
                                }
                                commitment_security
                            })
                            .collect();
                    }
                    Ok(already_committed)
                }
            }
        },
    )?;

    // If there is no entry for AVAILABLE_CAPITAL then the deposit goes in as the entry
    // If there is an entry then we update the entry by adding each type of fund coin to the appropriate available capital coin
    // Realistically, there will be only one type of coin in the fund.
    AVAILABLE_CAPITAL.update(
        deps.storage,
        sender.clone(),
        |available_capital| -> StdResult<Vec<Coin>> {
            match available_capital {
                None => Ok(funds),
                Some(mut available_capital) => {
                    for fund_coin in funds {
                        available_capital = available_capital
                            .into_iter()
                            .map(|mut capital_coin| {
                                if capital_coin.denom == fund_coin.denom {
                                    capital_coin.amount += fund_coin.amount;
                                }
                                capital_coin
                            })
                            .collect();
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
