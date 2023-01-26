use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
        state::STATE,
    },
    util::validate::{Validate, ValidateResult},
};

mod propose_subscription;

pub fn run(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
    match msg {
        ExecuteMsg::ProposeSubscription { initial_commitment } => {
            propose_subscription::propose_subscription(
                deps,
                &info.sender,
                env.contract.address.into_string(),
                state.subscription_code_id,
                &state.commitment_denom,
                &state.recovery_admin,
                initial_commitment,
            )
        }
    }
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
