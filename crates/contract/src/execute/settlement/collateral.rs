use cosmwasm_std::{Addr, Coin, Uint128};
use provwasm_std::AccessGrant;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LoanPoolMarkerCollateral {
    pub marker_address: Addr,
    pub marker_denom: String,
    pub share_count: Uint128,
    pub removed_permissions: Vec<AccessGrant>,
}
impl LoanPoolMarkerCollateral {
    fn new<S: Into<String>>(
        marker_address: Addr,
        marker_denom: S,
        share_count: u128,
        removed_permissions: &[AccessGrant],
    ) -> Self {
        Self {
            marker_address,
            marker_denom: marker_denom.into(),
            share_count: Uint128::new(share_count),
            removed_permissions: removed_permissions.to_owned(),
        }
    }
}
