use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub subscription_code_id: u64,
    pub recovery_admin: Addr,
    pub gp: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    ProposeSubscription { initial_commitment: Option<u64> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MyQueryResponse)]
    MyQuery {},
}

#[cw_serde]
pub struct MyQueryResponse {}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub struct SubInstantiateMsg {
    pub admin: Addr,
    pub lp: Addr,
    pub commitment_denom: String,
    pub initial_commitment: Option<u64>,
}
