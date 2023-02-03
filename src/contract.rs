use cosmwasm_std::{entry_point, Env, MessageInfo};

use crate::{
    core::aliases::{ProvDeps, ProvDepsMut, ProvQueryResponse, ProvTxResponse},
    core::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    execute, instantiate, migrate, query,
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
    if msg.requires_funds() && info.funds.is_empty() {
        return Err(crate::core::error::ContractError::MissingFunds {});
    } else if !msg.requires_funds() && !info.funds.is_empty() {
        return Err(crate::core::error::ContractError::UnexpectedFunds {});
    }
    instantiate::handle(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: ProvDeps, env: Env, msg: QueryMsg) -> ProvQueryResponse {
    msg.validate()?;
    query::handle(deps, env, msg)
}

#[entry_point]
pub fn execute(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    msg.validate()?;
    if msg.requires_funds() && info.funds.is_empty() {
        return Err(crate::core::error::ContractError::MissingFunds {});
    } else if !msg.requires_funds() && !info.funds.is_empty() {
        return Err(crate::core::error::ContractError::UnexpectedFunds {});
    }
    execute::handle(deps, env, info, msg)
}

#[entry_point]
pub fn migrate(deps: ProvDepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    msg.validate()?;
    migrate::handle(deps, env, msg)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
