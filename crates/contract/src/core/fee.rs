use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct Fee {
    pub recipient: Addr,
    pub amount: Coin,
}
