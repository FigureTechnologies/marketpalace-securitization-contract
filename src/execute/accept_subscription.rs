use cosmwasm_std::Response;

use crate::core::aliases::ProvTxResponse;

pub fn accept_subscription() -> ProvTxResponse {
    Ok(Response::new())
}
