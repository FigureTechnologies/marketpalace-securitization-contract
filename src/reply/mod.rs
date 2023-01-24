use cosmwasm_std::{Env, Reply, Response};

use crate::core::aliases::{ProvDepsMut, ProvTxResponse};

pub fn run(_deps: ProvDepsMut, _env: Env, _msg: Reply) -> ProvTxResponse {
    Ok(Response::new())
}
