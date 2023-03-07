use cosmwasm_std::{Addr, Env, Response, Uint128};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage,
    util::{is_contract_admin::is_contract_admin, migrate_contracts::migrate_contracts},
};

// We may need to do batching on this because of the large amount of securities
pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    sender: Addr,
    contracts: Vec<Addr>,
    contract_id: Uint128,
) -> ProvTxResponse {
    if !is_contract_admin(&deps, &env, sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if storage::state::is_migrating(deps.storage)? {
        return Err(ContractError::MigrationInProcess {});
    }

    let messages = migrate_contracts(deps.storage, &contracts, contract_id)?;

    Ok(Response::default()
        .add_attribute("action", "migrate_contracts")
        .add_submessages(messages))
}
