use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};

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

#[cw_serde]
pub struct AcceptedCommitment {
    pub lp: Addr,
    pub securities: Vec<SecurityCommitment>,
}

#[cw_serde]
pub struct ContributeLoanPools {
    pub originalOwner: Addr, // who owns this set of loan pools, this assumes a homogenous loan pools, i.e one owner owns all loan pools in the markers field
    pub markers: Vec<Addr>, // marker address for loan pools being contributed, usually will be only a set of 1
}
