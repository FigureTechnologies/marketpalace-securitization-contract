use cosmwasm_std::{
    testing::{mock_info, MockApi, MockStorage},
    to_json_binary, Addr, Coin, ContractInfoResponse, ContractResult, Env, OwnedDeps, QuerierResult,
    SubMsg, SystemError, SystemResult, Uint128, Uint64, WasmMsg, WasmQuery,
};
use provwasm_mocks::{mock_dependencies, ProvenanceMockQuerier};
use provwasm_std::ProvenanceQuery;

use crate::{
    contract::{execute, instantiate},
    core::{
        aliases::{ProvDepsMut, ProvSubMsg, ProvTxResponse},
        error::ContractError,
        msg::{Contract, ContractMigrateMsg, ExecuteMsg, InstantiateMsg, QueryMsg},
        security,
    },
};

pub fn test_init_message() -> InstantiateMsg {
    InstantiateMsg {
        batch_size: Uint128::new(2),
    }
}

pub fn test_remove_contracts_empty_message() -> ExecuteMsg {
    ExecuteMsg::RemoveContracts { contracts: vec![] }
}

pub fn test_remove_contracts_message() -> ExecuteMsg {
    ExecuteMsg::RemoveContracts {
        contracts: vec![
            Contract {
                address: Addr::unchecked("contract1"),
                uuid: "uuid1".to_string(),
            },
            Contract {
                address: Addr::unchecked("contract3"),
                uuid: "uuid3".to_string(),
            },
        ],
    }
}

pub fn test_add_contracts_message() -> ExecuteMsg {
    ExecuteMsg::AddContracts {
        contracts: vec![
            Contract {
                address: Addr::unchecked("contract1"),
                uuid: "uuid1".to_string(),
            },
            Contract {
                address: Addr::unchecked("contract2"),
                uuid: "uuid2".to_string(),
            },
            Contract {
                address: Addr::unchecked("contract3"),
                uuid: "uuid3".to_string(),
            },
        ],
    }
}

pub fn test_create_contract_message() -> ExecuteMsg {
    ExecuteMsg::CreateContract {
        init_msg: (test_create_contract_init_message()),
        code_id: Uint64::new(123),
        uuid: "uuid".to_string(),
    }
}

pub fn test_create_contract_init_message() -> security::InstantiateMsg {
    security::InstantiateMsg {
        gp: Addr::unchecked("gp"),
        securities: vec![],
        capital_denom: "denom".to_string(),
        settlement_time: None,
        fee: None,
    }
}

pub fn test_add_contracts_empty_message() -> ExecuteMsg {
    ExecuteMsg::AddContracts { contracts: vec![] }
}

pub fn test_migrate_contracts_message() -> ExecuteMsg {
    ExecuteMsg::MigrateContracts {
        contracts: vec![Addr::unchecked("contract1"), Addr::unchecked("contract3")],
        new_contract: Uint128::new(2),
    }
}

pub fn test_migrate_contracts_empty_message() -> ExecuteMsg {
    ExecuteMsg::MigrateContracts {
        contracts: vec![],
        new_contract: Uint128::new(2),
    }
}

pub fn test_migrate_all_contracts_message() -> ExecuteMsg {
    ExecuteMsg::MigrateAllContracts {
        new_contract: Uint128::new(2),
    }
}

pub fn test_modify_batch_size_message() -> ExecuteMsg {
    ExecuteMsg::ModifyBatchSize {
        batch_size: Uint128::new(7),
    }
}

pub fn migrate_message(contract: Addr, contract_id: Uint128, message_id: u64) -> ProvSubMsg {
    let msg = WasmMsg::Migrate {
        contract_addr: contract.to_string(),
        new_code_id: contract_id.u128() as u64,
        msg: to_json_binary(&ContractMigrateMsg {}).unwrap(),
    };
    SubMsg::reply_always(msg, message_id)
}

pub fn instantiate_contract_message(owner: Addr, code_id: Uint64) -> ProvSubMsg {
    let msg = WasmMsg::Instantiate {
        admin: Some(owner.to_string()),
        code_id: code_id.u64(),
        msg: to_json_binary(&test_create_contract_init_message()).unwrap(),
        funds: vec![],
        label: format!("securitization"),
    };
    SubMsg::reply_on_success(msg, 0)
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
                SystemResult::Ok(ContractResult::Ok(to_json_binary(&res).unwrap()))
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
