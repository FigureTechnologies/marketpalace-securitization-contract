use cosmwasm_std::{Addr, CosmosMsg, Env, MessageInfo, Response, StdResult};
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
        state::{State, SECURITY_TYPES, STATE},
    },
    util::validate::{Validate, ValidateResult},
};

pub fn run(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::new(info.sender);
    STATE.save(deps.storage, &state)?;

    // Create the markers
    let mut messages: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();
    for security in msg.securities {
        let commitment_name = security.get_commitment_name(&env.contract.address);
        let investment_name = security.get_investment_name(&env.contract.address);
        let mut commitment_marker = new_active_marker(
            env.contract.address.clone(),
            &commitment_name,
            security.amount,
        )?;
        messages.append(&mut commitment_marker);
        let mut investment_marker = new_active_marker(
            env.contract.address.clone(),
            &investment_name,
            security.amount,
        )?;
        messages.append(&mut investment_marker);
        SECURITY_TYPES.save(deps.storage, security.name.clone(), &security)?;
    }

    Ok(Response::default().add_messages(messages))
}

fn new_active_marker(
    owner: Addr,
    denom: &String,
    amount: u128,
) -> StdResult<Vec<CosmosMsg<ProvenanceMsg>>> {
    let permissions = vec![
        MarkerAccess::Admin,
        MarkerAccess::Mint,
        MarkerAccess::Burn,
        MarkerAccess::Withdraw,
    ];
    Ok(vec![
        create_marker(amount, denom.clone(), MarkerType::Coin)?,
        grant_marker_access(denom, owner, permissions)?,
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
