use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryPendingCommitmentsResponse},
    storage,
};

pub fn query_pending_commitments(storage: &dyn Storage) -> ProvQueryResponse {
    let commitments = storage::commits::get_pending(storage);
    let response = QueryPendingCommitmentsResponse { commitments };
    Ok(to_binary(&response)?)
}
