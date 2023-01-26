use cosmwasm_std::{CosmosMsg, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use provwasm_std::{
    activate_marker, create_marker, finalize_marker, grant_marker_access, MarkerAccess, MarkerType,
    ProvenanceMsg,
};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::InstantiateMsg,
        state::{State, STATE},
    },
    util::validate::{Validate, ValidateResult},
};

pub fn run(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::new(
        &env.contract.address,
        msg.subscription_code_id,
        &msg.recovery_admin,
        &info.sender,
    );
    STATE.save(deps.storage, &state)?;
    Ok(Response::default().add_messages(new_active_marker(&env, &state.commitment_denom)?))
}

fn new_active_marker(env: &Env, denom: &String) -> StdResult<Vec<CosmosMsg<ProvenanceMsg>>> {
    let permissions = vec![
        MarkerAccess::Admin,
        MarkerAccess::Mint,
        MarkerAccess::Burn,
        MarkerAccess::Withdraw,
    ];
    Ok(vec![
        create_marker(0, denom.clone(), MarkerType::Coin)?,
        grant_marker_access(denom, env.contract.address.clone(), permissions)?,
        finalize_marker(denom)?,
        activate_marker(denom)?,
    ])
}

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        // Add validation checks
        Ok(())
    }
}
