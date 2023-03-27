use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint64};
use cw2::ContractVersion;

use super::security;

#[cw_serde]
pub struct InstantiateMsg {
    pub batch_size: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddContracts {
        contracts: Vec<Addr>,
    },
    RemoveContracts {
        contracts: Vec<Addr>,
    },
    MigrateContracts {
        contracts: Vec<Addr>,
        new_contract: Uint128,
    },
    MigrateAllContracts {
        new_contract: Uint128,
    },
    ModifyBatchSize {
        batch_size: Uint128,
    },
    CreateContract {
        init_msg: security::InstantiateMsg,
        code_id: Uint64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryVersionResponse)]
    QueryVersion {},

    #[returns(QueryStateResponse)]
    QueryState {},

    #[returns(QueryContractsResponse)]
    QueryContracts {},
}

#[cw_serde]
pub struct QueryVersionResponse {
    pub contract_version: ContractVersion,
}

#[cw_serde]
pub struct QueryStateResponse {
    pub batch_size: Uint128,
    pub migrating: bool,
}

#[cw_serde]
pub struct QueryContractsResponse {
    pub contracts: Vec<Addr>,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ContractMigrateMsg {}
