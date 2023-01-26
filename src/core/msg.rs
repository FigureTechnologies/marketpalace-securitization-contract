use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub gp: Addr,
    pub securities: Vec<Security>,
}

#[cw_serde]
pub enum ExecuteMsg {
    ProposeSubscription { securities: Vec<SecurityCommitment> },
    AcceptSubscription,
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
}

impl Security {
    pub fn get_commitment_name(&self, contract: &Addr) -> String {
        format! {"{}.{}.commitment", contract, self.name}
    }

    pub fn get_investment_name(&self, contract: &Addr) -> String {
        format! {"{}.{}.investment", contract, self.name}
    }
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
