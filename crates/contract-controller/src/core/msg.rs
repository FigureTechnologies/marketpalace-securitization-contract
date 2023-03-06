use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw2::ContractVersion;

#[cw_serde]
pub struct InstantiateMsg {
    pub batch_size: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddContracts { contracts: Vec<Addr> },
    RemoveContracts { contracts: Vec<Addr> },
    MigrateContracts { new_contract: Uint128 },
    ModifyBatchSize { batch_size: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryVersionResponse)]
    QueryVersion {},
}

#[cw_serde]
pub struct QueryStateResponse {}

#[cw_serde]
pub struct QueryVersionResponse {
    pub contract_version: ContractVersion,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ContractMigrateMsg {}
