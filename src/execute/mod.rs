use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
        state::STATE,
    },
    util::validate::{Validate, ValidateResult},
};

use self::accept_subscriptions::accept_subscriptions;

mod accept_subscriptions;
mod propose_subscription;

pub fn run(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    let _state = STATE.load(deps.storage)?;
    match msg {
        ExecuteMsg::ProposeSubscription { securities } => {
            propose_subscription::propose_subscription(deps, info.sender, securities)
        }
        ExecuteMsg::AcceptSubscription { subscriptions } => {
            accept_subscriptions(env, deps, subscriptions)
        }
    }
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
