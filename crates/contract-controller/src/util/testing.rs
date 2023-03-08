use std::marker::PhantomData;

use cosmwasm_std::{
    testing::{mock_info, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    to_binary, Addr, Coin, ContractInfoResponse, ContractResult, Env, OwnedDeps, QuerierResult,
    SystemError, SystemResult, Uint128, WasmQuery,
};
use provwasm_mocks::{mock_dependencies, ProvenanceMockQuerier};
use provwasm_std::ProvenanceQuery;

use crate::{
    contract::{execute, instantiate},
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    },
};

pub fn test_init_message() -> InstantiateMsg {
    InstantiateMsg {
        batch_size: Uint128::new(2),
    }
}

pub fn test_add_contracts_message() -> ExecuteMsg {
    ExecuteMsg::AddContracts {
        contracts: vec![
            Addr::unchecked("contract1"),
            Addr::unchecked("contract2"),
            Addr::unchecked("contract3"),
        ],
    }
}

pub fn instantiate_contract(deps: ProvDepsMut, env: Env) -> ProvTxResponse {
    let info = mock_info("sender", &[]);
    let msg = test_init_message();
    instantiate(deps, env, info, msg)
}

pub fn add_contracts(deps: ProvDepsMut, env: Env) -> ProvTxResponse {
    let info = mock_info("admin", &[]);
    let msg = test_add_contracts_message();
    execute(deps, env, info, msg)
}

pub fn get_test_admin(deps: &ProvDepsMut, env: &Env) -> Result<Addr, ContractError> {
    let contract = deps
        .querier
        .query_wasm_contract_info(env.contract.address.clone())?;
    Ok(Addr::unchecked(contract.admin.unwrap()))
}

pub fn create_admin_deps(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery> {
    let mut deps = mock_dependencies(contract_balance);
    let querier: &mut ProvenanceMockQuerier = &mut deps.querier;

    let handler = Box::from(|request: &WasmQuery| -> QuerierResult {
        let err = match request {
            WasmQuery::Smart { contract_addr, .. } => {
                SystemResult::Err(SystemError::NoSuchContract {
                    addr: contract_addr.clone(),
                })
            }
            WasmQuery::Raw { contract_addr, .. } => {
                SystemResult::Err(SystemError::NoSuchContract {
                    addr: contract_addr.clone(),
                })
            }
            WasmQuery::ContractInfo {
                contract_addr: _, ..
            } => {
                let mut res = ContractInfoResponse::default();
                res.admin = Some("admin".to_string());
                SystemResult::Ok(ContractResult::Ok(to_binary(&res).unwrap()))
            }
            #[cfg(feature = "cosmwasm_1_2")]
            WasmQuery::CodeInfo { code_id, .. } => {
                SystemResult::Err(SystemError::NoSuchCode { code_id: *code_id })
            }
            _ => SystemResult::Err(SystemError::Unknown {}),
        };
        err
    });

    querier.base.update_wasm(handler);
    deps
}

pub fn create_test_query_contracts() -> QueryMsg {
    QueryMsg::QueryContracts {}
}

pub fn create_test_query_state() -> QueryMsg {
    QueryMsg::QueryState {}
}

pub fn create_test_query_verison() -> QueryMsg {
    QueryMsg::QueryVersion {}
}
