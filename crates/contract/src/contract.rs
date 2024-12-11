use cosmwasm_std::{entry_point, Env, MessageInfo};

use crate::{
    core::aliases::{ProvDeps, ProvDepsMut, ProvQueryResponse, ProvTxResponse},
    core::{
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    },
    execute, instantiate, migrate, query,
    util::validate::Validate,
};

/// The entry point used when an external address instantiates a stored code wasm payload of this
/// contract on the Provenance Blockchain.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `msg` A custom instantiation message defined by this contract for creating the initial
/// configuration used by the contract.
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

/// The entry point used when an external address attemtpts to retrieve information from the contract.
/// Allows access to the internal storage information.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `msg` A custom query message enum defined by this contract to allow multiple different results
/// to be determined for this route.
// #[entry_point]
// pub fn query(deps: ProvDeps, env: Env, msg: QueryMsg) -> ProvQueryResponse {
//     msg.validate()?;
//     query::router::route(deps, env, msg)
// }

/// The entry point used when an external address initiates a process defined in the
/// contract.  This defines the primary purposes of this contract, like the onboarding and
/// verification processes, as well as allowing the administrator address to make changes to the
/// contract's internal configuration.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `msg` A custom execution message enum defined by this contract to allow multiple different
/// processes to be defined for the singular execution route entry point allowed by the
/// cosmwasm framework.
#[entry_point]
pub fn execute(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    // msg.validate()?;
    // msg.validate_msg_funds(&info.funds)?;
    execute::router::route(deps, env, info, msg)
}

/// The entry point used when migrating a live contract instance to a new code instance, or to
/// refresh the contract with an existing matching codebase for the purpose of running migration
/// options.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * msg` A custom migrate message enum defined by this contract to allow multiple different
/// results of invoking the migrate endpoint.
#[entry_point]
pub fn migrate(deps: ProvDepsMut, env: Env, msg: MigrateMsg) {
}
//     msg.validate()?;
//     let res = migrate::handler::handle(&deps, env, msg);
//     cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
//     res
// }

