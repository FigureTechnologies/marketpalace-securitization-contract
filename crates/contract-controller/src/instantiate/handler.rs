use cosmwasm_std::{Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    constants::{CONTRACT_NAME, CONTRACT_VERSION},
    msg::InstantiateMsg,
};

pub fn handle(
    deps: ProvDepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default().add_attribute("action", "init"))
}

#[cfg(test)]
mod tests {}
