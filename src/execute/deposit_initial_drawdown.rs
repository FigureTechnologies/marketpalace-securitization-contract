use cosmwasm_std::{Addr, Coin, Order, Response, StdResult};

use crate::{
    commitment::CommitmentState,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::SecurityCommitment,
        state::{AVAILABLE_CAPITAL, COMMITS, PAID_IN_CAPITAL, SECURITIES_MAP},
    },
};

pub fn deposit_initial_drawdown(
    deps: ProvDepsMut,
    sender: Addr,
    funds: Vec<Coin>,
    initial_drawdown: Vec<SecurityCommitment>,
) -> ProvTxResponse {
    let commitment = COMMITS.load(deps.storage, sender.clone())?;
    if commitment.state != CommitmentState::PENDING {
        // TODO
        // Throw an error
    }

    if !drawdown_met(&deps, &initial_drawdown) {
        // TODO
        // Throw an error
    }

    // Check that the correct of funds passed in exactly match expected
    let expected_funds = calculate_funds(&deps, &initial_drawdown);
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

        // TODO Do we need to check maximum?
    }

    true
}

// This allows us to calculate funds that has multiple types of coins
// The security type is guaranteed to be there because of the previous function check
// We calculate the funds by doing the following...
// For each security we calculate its cost by getting its commitment amount and multiplying it by the number of units
// Then add it to the end of the list if the denom does not exist in sum already
// If it already exists in sum then we create a new sum list with this added to the already existing coin in the list.
fn calculate_funds(deps: &ProvDepsMut, initial_drawdown: &[SecurityCommitment]) -> Vec<Coin> {
    let mut sum: Vec<Coin> = vec![];

    for security_commitment in initial_drawdown {
        let security = SECURITIES_MAP
            .load(deps.storage, security_commitment.name.clone())
            .unwrap();

        let cost = Coin::new(
            security_commitment.amount * security.price_per_unit.amount.u128(),
            security.price_per_unit.denom.clone(),
        );

        if !sum
            .iter()
            .any(|coin| coin.denom == security.price_per_unit.denom)
        {
            sum.push(cost);
        } else {
            sum = sum
                .into_iter()
                .map(|mut coin| {
                    if coin.denom == security.price_per_unit.denom {
                        coin.amount += cost.amount;
                    }
                    coin
                })
                .collect();
        }
    }

    sum
}
