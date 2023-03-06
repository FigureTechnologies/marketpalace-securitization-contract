use cosmwasm_std::{Addr, Env};

use crate::core::{aliases::ProvDepsMut, error::ContractError};

pub fn is_contract_admin(deps: &ProvDepsMut, env: &Env, addr: Addr) -> Result<bool, ContractError> {
    let admin = get_contract_admin(deps, env)?;
    Ok(admin == addr)
}

fn get_contract_admin(deps: &ProvDepsMut, env: &Env) -> Result<Addr, ContractError> {
    let response = deps
        .querier
        .query_wasm_contract_info(env.contract.address.clone())?;
    Ok(Addr::unchecked(response.admin.unwrap()))
}
