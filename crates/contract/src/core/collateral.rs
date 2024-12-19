use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg, Uint128};
use provwasm_std::types::provenance::marker::v1::{Access, AccessGrant};
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
    pub removed_permissions: Vec<AccessGrantSerializable>,
}

#[cw_serde]
pub struct AccessGrantSerializable {
    pub address: String,
    pub permissions: Vec<MarkerAccess>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerAccess {
    Admin,
    Burn,
    Deposit,
    Delete,
    Mint,
    Transfer,
    ForceTransfer,
    Unspecified,
    Withdraw,
}

impl From<MarkerAccess> for Access {
    fn from(value: MarkerAccess) -> Self {
        match value {
            MarkerAccess::Unspecified => Access::Unspecified,
            MarkerAccess::Admin => Access::Admin,
            MarkerAccess::Burn => Access::Burn,
            MarkerAccess::Deposit => Access::Deposit,
            MarkerAccess::Delete => Access::Delete,
            MarkerAccess::Mint => Access::Mint,
            MarkerAccess::Transfer => Access::Transfer,
            MarkerAccess::ForceTransfer => Access::ForceTransfer,
            MarkerAccess::Withdraw => Access::Withdraw,
        }
    }
}

impl From<Access> for MarkerAccess {
    fn from(value: Access) -> Self {
        match value {
            Access::Unspecified => MarkerAccess::Unspecified,
            Access::Admin => MarkerAccess::Admin,
            Access::Burn => MarkerAccess::Burn,
            Access::Deposit => MarkerAccess::Deposit,
            Access::Delete => MarkerAccess::Delete,
            Access::Mint => MarkerAccess::Mint,
            Access::Transfer => MarkerAccess::Transfer,
            Access::Withdraw => MarkerAccess::Withdraw,
            Access::ForceTransfer => MarkerAccess::ForceTransfer,
        }
    }
}

impl From<AccessGrant> for AccessGrantSerializable {
    fn from(access_grant: AccessGrant) -> Self {
        AccessGrantSerializable {
            address: access_grant.address,
            permissions: access_grant
                .permissions
                .into_iter()
                .map(|p| MarkerAccess::from(Access::from_i32(p).unwrap()))
                .collect(),
        }
    }
}

impl From<AccessGrantSerializable> for AccessGrant {
    fn from(serializable: AccessGrantSerializable) -> Self {
        AccessGrant {
            address: serializable.address,
            permissions: serializable
                .permissions
                .into_iter()
                .map(|p| Access::from(p).into())
                .collect(),
        }
    }
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
            removed_permissions: removed_permissions
                .into_iter()
                .map(AccessGrantSerializable::from)
                .collect(),
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
    pub messages: Vec<CosmosMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Holds information about a loan pool removal.
pub struct LoanPoolRemovalData {
    /// The collateral to be deleted from state
    pub collateral: LoanPoolMarkerCollateral,
    /// The Provenance messages associated with the loan.
    pub messages: Vec<CosmosMsg>,
}
