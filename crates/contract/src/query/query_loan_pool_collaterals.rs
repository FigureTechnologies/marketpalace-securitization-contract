use crate::core::aliases::ProvQueryResponse;
use crate::core::msg::QueryLoanPoolCollateralResponse;
use crate::storage::loan_pool_collateral::get_all_states;
use cosmwasm_std::{to_binary, Storage};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let loan_pool_collaterals = get_all_states(storage);
    let response = QueryLoanPoolCollateralResponse {
        collaterals: loan_pool_collaterals,
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {}
