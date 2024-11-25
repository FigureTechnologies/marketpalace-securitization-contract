use cosmwasm_std::{Addr, CosmosMsg, Uint128};
use provwasm_std::types::provenance::marker::v1::AccessGrant;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LoanPoolMarkerCollateral {
    pub marker_address: Addr,
    pub marker_denom: String,
    pub share_count: Uint128,
    pub original_contributor: Addr,
    // this is the address with ADMIN privileges that added the marker to the securitization.
    pub removed_permissions: Vec<AccessGrant>,
}

impl LoanPoolMarkerCollateral {
    pub(crate) fn new<S: Into<String>>(
        marker_address: Addr,
        marker_denom: S,
        share_count: u128,
        original_owner: Addr,
        removed_permissions: Vec<AccessGrant>,
    ) -> Self {
        Self {
            marker_address,
            marker_denom: marker_denom.into(),
            share_count: Uint128::new(share_count),
            original_contributor: original_owner,
            removed_permissions,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Holds information about a loan pool addition.
pub struct LoanPoolAdditionData {
    /// The collateral being added to the loan.
    pub collateral: LoanPoolMarkerCollateral,
    /// The Provenance messages associated with the loan.
    pub messages: Vec<CosmosMsg<ProvenanceMsg>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Holds information about a loan pool removal.
pub struct LoanPoolRemovalData {
    /// The collateral to be deleted from state
    pub collateral: LoanPoolMarkerCollateral,
    /// The Provenance messages associated with the loan.
    pub messages: Vec<CosmosMsg<ProvenanceMsg>>,
}
