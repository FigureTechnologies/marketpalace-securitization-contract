use cosmwasm_std::{Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::InstantiateMsg,
    },
    storage::{self, state::State},
};

pub fn handle(
    deps: ProvDepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::new(msg.batch_size.u128());
    storage::state::set(deps.storage, &state)?;
    Ok(Response::default().add_attribute("action", "init"))
}

#[cfg(test)]
mod tests {
    
}
