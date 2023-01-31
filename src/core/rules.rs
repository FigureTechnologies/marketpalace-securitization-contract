use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Eq)]
pub enum InvestmentVehicleRule {
    SettlementDate(SettlementDate),
}

#[cw_serde]
#[derive(Eq)]
pub struct SettlementDate {}
