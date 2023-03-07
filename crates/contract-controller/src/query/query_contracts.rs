use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryContractsResponse},
    storage,
};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let response = QueryContractsResponse {
        contracts: storage::contract::list(storage),
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {}
