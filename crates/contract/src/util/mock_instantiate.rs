use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Addr, DepsMut, Uint128, Uint64};
use provwasm_std::ProvenanceQuery;
use crate::core::fee::Fee;
use crate::core::msg::InstantiateMsg;
use crate::core::security::Security;
use crate::util::testing::{create_test_securities, instantiate_contract};

pub const DEFAULT_ADMIN_ADDRESS: &str = "contract_admin";
pub const DEFAULT_CONTRACT_BIND_NAME: &str = "contract_bind_name";
pub const DEFAULT_CONTRACT_NAME: &str = "contract_name";

pub struct TestInstantiate {
    pub gp: Addr,
    pub securities: Vec<Security>,
    pub capital_denom: String,
    pub settlement_time: Option<Uint64>,
    pub fee: Option<Fee>,
}
impl Default for TestInstantiate {
    fn default() -> Self {
        Self {
            gp: Addr::unchecked("gp"),
            securities: create_test_securities(),
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        }
    }
}
impl TestInstantiate {
    pub fn to_instantiate_msg(self) -> InstantiateMsg {
        InstantiateMsg {
            gp: self.gp,
            securities: self.securities,
            capital_denom: self.capital_denom,
            settlement_time: self.settlement_time,
            fee: self.fee,
        }
    }
}

pub fn default_instantiate(deps: DepsMut<ProvenanceQuery>) {
    test_instantiate(deps,)
}

pub fn test_instantiate(deps: DepsMut<ProvenanceQuery>) {
    instantiate_contract(
        deps
    )
    .expect("expected instantiation to succeed");
}
