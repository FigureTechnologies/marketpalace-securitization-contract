use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryMsg,
};

use super::{query_contracts, query_state, query_version};

pub fn route(deps: ProvDeps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryVersion {} => query_version::handle(deps.storage),
        QueryMsg::QueryState {} => query_state::handle(deps.storage),
        QueryMsg::QueryContracts {} => query_contracts::handle(deps.storage),
    }
}

#[cfg(test)]
mod tests {}
