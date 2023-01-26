use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
        state::STATE,
    },
    util::validate::{Validate, ValidateResult},
};

use self::accept_subscription::accept_subscription;

mod accept_subscription;
mod propose_subscription;

pub fn run(deps: ProvDepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    let _state = STATE.load(deps.storage)?;
    match msg {
        ExecuteMsg::ProposeSubscription { securities } => {
            propose_subscription::propose_subscription(deps, info.sender, securities)
        }
        ExecuteMsg::AcceptSubscription {} => accept_subscription(),
    }
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
