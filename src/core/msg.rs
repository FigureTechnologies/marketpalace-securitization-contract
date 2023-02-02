use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use super::{
    rules::InvestmentVehicleRule,
    security::{Security, SecurityCommitment},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub gp: Addr,
    pub securities: Vec<Security>,
    pub capital_denom: String,
    pub rules: Vec<InvestmentVehicleRule>,
}

#[cw_serde]
pub enum ExecuteMsg {
    ProposeCommitment { securities: Vec<SecurityCommitment> },
    AcceptCommitment { commitments: Vec<Addr> },
    DepositCommitment { securities: Vec<SecurityCommitment> },
    WithdrawCommitments {},
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
