use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QuerySecuritizationsResponse},
    storage::{self},
};

pub fn query_securitizations(
    storage: &dyn Storage,
    security_names: Vec<String>,
) -> ProvQueryResponse {
    let mut securities = vec![];

    for security in security_names {
        securities.push(storage::securities::get(storage, security)?);
    }

    let response = QuerySecuritizationsResponse { securities };

    Ok(to_binary(&response)?)
}
