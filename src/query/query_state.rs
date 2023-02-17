use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryStateResponse},
    storage,
};

pub fn query_state(storage: &dyn Storage) -> ProvQueryResponse {
    let state = storage::state::get(storage)?;
    let securities = storage::securities::get_security_types(storage);
    let response = QueryStateResponse {
        gp: state.gp,
        securities,
        capital_denom: state.capital_denom,
        rules: state.rules,
    };
    Ok(to_binary(&response)?)
}
