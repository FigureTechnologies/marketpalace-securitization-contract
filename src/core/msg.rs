use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use super::security::{Security, SecurityCommitment};

#[cw_serde]
pub struct InstantiateMsg {
    pub gp: Addr,
    pub securities: Vec<Security>,
    pub capital_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ProposeCommitment { securities: Vec<SecurityCommitment> },
    AcceptCommitment { commitments: Vec<Addr> },
    DepositInitialDrawdown { securities: Vec<SecurityCommitment> },
    WithdrawCapital {},
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
