use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
#[derive(Eq)]
pub struct Security {
    pub name: String,
    pub amount: u128,
    pub security_type: SecurityType,

    pub minimum_amount: u128,
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
