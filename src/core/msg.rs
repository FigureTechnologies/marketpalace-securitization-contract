use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

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

// TODO Extract these out

#[cw_serde]
#[derive(Eq)]
pub struct Security {
    pub name: String,
    pub amount: u128,
    pub minimum_amount: u128,
    pub security_type: SecurityType,
    pub price_per_unit: Coin,
}

#[cw_serde]
#[derive(Eq)]
pub enum SecurityType {
    Fund,
    Primary,
    Tranche,
}

#[cw_serde]
pub struct SecurityCommitment {
    pub name: String,
    pub amount: u128,
}
