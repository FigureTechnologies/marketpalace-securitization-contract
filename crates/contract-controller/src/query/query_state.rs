use cosmwasm_std::{to_binary, Storage, Uint128};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryStateResponse},
    storage,
};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let state = storage::state::get(storage)?;
    let response = QueryStateResponse {
        batch_size: Uint128::new(state.batch_size),
        migrating: state.migrating,
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {}
