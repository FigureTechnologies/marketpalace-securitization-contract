use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
#[derive(Eq)]
pub enum InvestmentVehicleRule {
    SettlementTime(Uint64),
}
