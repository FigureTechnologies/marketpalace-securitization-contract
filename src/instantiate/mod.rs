use cosmwasm_std::{Addr, CosmosMsg, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use provwasm_std::{
    activate_marker, create_marker, finalize_marker, grant_marker_access, withdraw_coins,
    MarkerAccess, MarkerType, ProvenanceMsg,
};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::InstantiateMsg,
        state::{State, SECURITIES_MAP, STATE},
    },
    util::{
        to,
        validate::{Validate, ValidateResult},
    },
};

pub fn run(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::new(info.sender, msg.capital_denom);
    STATE.save(deps.storage, &state)?;

    // Create the markers
    let mut messages: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();
    for security in &msg.securities {
        let investment_name =
            to::security_to_investment_name(&security.name, &env.contract.address);
        let mut investment_marker =
            new_active_marker(env.contract.address.clone(), &investment_name, 0)?;
        messages.append(&mut investment_marker);
        SECURITIES_MAP.save(deps.storage, security.name.clone(), security)?;
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
        grant_marker_access(denom, owner.clone(), permissions)?,
        finalize_marker(denom)?,
        activate_marker(denom)?,
        withdraw_coins(denom, amount, denom, owner)?,
    ])
}

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        // Add validation checks
        Ok(())
    }
}
