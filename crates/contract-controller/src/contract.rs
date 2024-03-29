use cosmwasm_std::{entry_point, Env, MessageInfo, Reply};

use crate::{
    core::aliases::{ProvDeps, ProvDepsMut, ProvQueryResponse, ProvTxResponse},
    core::{
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    },
    execute, instantiate, migrate, query, reply,
    util::validate::Validate,
};

#[entry_point]
pub fn instantiate(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    msg.validate()?;
    msg.validate_msg_funds(&info.funds)?;
    instantiate::handler::handle(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: ProvDeps, env: Env, msg: QueryMsg) -> ProvQueryResponse {
    msg.validate()?;
    query::router::route(deps, env, msg)
}

#[entry_point]
pub fn execute(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    msg.validate()?;
    msg.validate_msg_funds(&info.funds)?;
    execute::router::route(deps, env, info, msg)
}

#[entry_point]
pub fn migrate(deps: ProvDepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    msg.validate()?;
    let res = migrate::handler::handle(&deps, env, msg);
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    res
}

#[entry_point]
pub fn reply(deps: ProvDepsMut, env: Env, reply: Reply) -> ProvTxResponse {
    reply::handler::handle(deps, env, reply)
}
