use cosmwasm_std::{to_binary, Addr, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryInvestorResponse},
    storage,
};

pub fn query_investor(storage: &dyn Storage, lp: Addr) -> ProvQueryResponse {
    let commitment = storage::commits::get(storage, lp.clone())?;
    let paid_in_capital = storage::paid_in_capital::get(storage, lp);
    let response = QueryInvestorResponse {
        commitment,
        paid_in_capital,
    };
    Ok(to_binary(&response)?)
}
