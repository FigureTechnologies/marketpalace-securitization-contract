use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};
use cw2::ContractVersion;

use crate::execute::settlement::commitment::Commitment;

use super::{
    fee::Fee,
    security::{Security, SecurityCommitment},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub gp: Addr,
    pub securities: Vec<Security>,
    pub capital_denom: String,
    pub settlement_time: Option<Uint64>,
    pub fee: Option<Fee>,
}

#[cw_serde]
pub enum ExecuteMsg {
    ProposeCommitment { securities: Vec<SecurityCommitment> },
    AcceptCommitment { commitments: Vec<Addr> },
    DepositCommitment { securities: Vec<SecurityCommitment> },
    WithdrawCommitment { lp: Addr },
    WithdrawAllCommitments {},
    UpdateSettlementTime { settlement_time: Option<Uint64> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryInvestorResponse)]
    QueryInvestor { investor: Addr },

    #[returns(QueryPendingCommitmentsResponse)]
    QueryPendingCommitments {},

    #[returns(QuerySecuritizationsResponse)]
    QuerySecuritizations { securities: Vec<String> },

    #[returns(QueryStateResponse)]
    QueryState {},

    #[returns(QueryVersionResponse)]
    QueryVersion {},
}

#[cw_serde]
pub struct QueryInvestorResponse {
    pub commitment: Commitment,
    pub paid_in_capital: Vec<SecurityCommitment>,
}

#[cw_serde]
pub struct QueryPendingCommitmentsResponse {
    pub commitments: Vec<Commitment>,
}

#[cw_serde]
pub struct QuerySecuritizationsResponse {
    pub securities: Vec<Security>,
}

#[cw_serde]
pub struct QueryStateResponse {
    pub gp: Addr,
    pub securities: Vec<String>,
    pub capital_denom: String,
    pub settlement_time: Option<Uint64>,
}

#[cw_serde]
pub struct QueryVersionResponse {
    pub contract_version: ContractVersion,
}

#[cw_serde]
pub struct MigrateMsg {}
