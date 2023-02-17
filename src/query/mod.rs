use cosmwasm_std::{Coin, Env};

use crate::{
    core::{
        aliases::{ProvDeps, ProvQueryResponse},
        msg::QueryMsg,
    },
    util::validate::{Validate, ValidateResult},
};

mod query_investor;
mod query_pending_commitments;
mod query_securitizations;
mod query_state;
mod query_version;

pub fn handle(deps: ProvDeps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryInvestor { investor } => {
            query_investor::query_investor(deps.storage, investor)
        }
        QueryMsg::QueryPendingCommitments {} => {
            query_pending_commitments::query_pending_commitments(deps.storage)
        }
        QueryMsg::QuerySecuritizations { securities } => {
            query_securitizations::query_securitizations(deps.storage, securities)
        }
        QueryMsg::QueryState {} => query_state::query_state(deps.storage),
        QueryMsg::QueryVersion {} => query_version::query_version(deps.storage),
    }
}

impl Validate for QueryMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
