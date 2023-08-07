use cosmwasm_std::{Addr, Coin, CosmosMsg, Uint128};
use provwasm_std::{AccessGrant, ProvenanceMsg};
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
    pub(crate) fn new<S: Into<String>>(
        marker_address: Addr,
        marker_denom: S,
        share_count: u128,
        removed_permissions: Vec<AccessGrant>,
    ) -> Self {
        Self {
            marker_address,
            marker_denom: marker_denom.into(),
            share_count: Uint128::new(share_count),
            removed_permissions: removed_permissions.to_owned(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoanPoolMarkers {
    pub collaterals: Vec<LoanPoolMarkerCollateral>,
}

impl LoanPoolMarkers {
    pub(crate) fn new(collaterals: Vec<LoanPoolMarkerCollateral>) -> Self {
        Self { collaterals }
    }
}


/// Holds information about a loan pool addition.
pub struct LoanPoolAdditionData {
    /// The collateral being added to the loan.
    pub collateral: LoanPoolMarkerCollateral,
    /// The Provenance messages associated with the loan.
    pub messages: Vec<CosmosMsg<ProvenanceMsg>>,
}
