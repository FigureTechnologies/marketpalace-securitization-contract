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
    pub markers: Vec<String>, // marker denom's for loan pools being contributed.
}

#[cw_serde]
pub struct WithdrawLoanPools {
    pub markers: Vec<String>, // marker denom's for loan pools being withdrawn, done by the GP
}

#[cw_serde]
pub struct LoanPoolContributors {
    pub addresses: Vec<Addr>, // white list of addresses allowed to contribute loan pols to the securitization
}

#[cw_serde]
pub struct RemoveLoanPoolContributors {
    pub addresses: Vec<Addr>, // white list of addresses allowed to contribute loan pols to the securitization
}
