use cosmwasm_std::{to_binary, Storage};
use cw2::get_contract_version;

use crate::core::{aliases::ProvQueryResponse, msg::QueryVersionResponse};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let response = QueryVersionResponse {
        contract_version: get_contract_version(storage)?,
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {}
