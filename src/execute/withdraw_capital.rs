use cosmwasm_std::{Addr, Response};

use crate::core::aliases::ProvTxResponse;

pub fn withdraw_capital(sender: &Addr) -> ProvTxResponse {
    Ok(Response::default())
}
