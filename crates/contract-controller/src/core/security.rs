use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128, Uint64};

#[cw_serde]
pub struct InstantiateMsg {
    pub gp: Addr,
    pub securities: Vec<Security>,
    pub capital_denom: String,
    pub settlement_time: Option<Uint64>,
    pub fee: Option<Fee>,
}

#[cw_serde]
pub struct Fee {
    pub recipient: Option<Addr>,
    pub amount: Coin,
}

#[cw_serde]
#[derive(Eq)]
pub struct Security {
    pub name: String,
    pub amount: Uint128,
    pub security_type: SecurityType,
    pub minimum_amount: Uint128,
    pub price_per_unit: Coin,
}

#[cw_serde]
#[derive(Eq)]
pub struct FundSecurity {}

#[cw_serde]
#[derive(Eq)]
pub struct PrimarySecurity {}

#[cw_serde]
#[derive(Eq)]
pub struct TrancheSecurity {}

#[cw_serde]
#[derive(Eq)]
pub enum SecurityType {
    Fund(FundSecurity),
    Primary(PrimarySecurity),
    Tranche(TrancheSecurity),
}

#[cw_serde]
pub struct SecurityCommitment {
    pub name: String,
    pub amount: Uint128,
}
