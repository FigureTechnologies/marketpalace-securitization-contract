use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cosmwasm_std::{coins, Addr, Coin, Decimal, Uint128};
use provwasm_std::types::cosmos::auth::v1beta1::BaseAccount;
use provwasm_std::types::provenance::marker::v1::{Access, AccessGrant, MarkerAccount, MarkerStatus, MarkerType};
use crate::util::provenance_utilities::Marker;

pub const DEFAULT_MARKER_ADDRESS: &str = "marker_address";
pub const DEFAULT_MARKER_HOLDINGS: u128 = 100;
pub const DEFAULT_MARKER_DENOM: &str = "markerdenom";

pub struct MockMarker {
    pub address: Addr,
    pub coins: Vec<Coin>,
    pub account_number: u64,
    pub sequence: u64,
    pub manager: String,
    pub permissions: Vec<AccessGrant>,
    pub status: MarkerStatus,
    pub denom: String,
    pub total_supply: Uint128,
    pub marker_type: MarkerType,
    pub supply_fixed: bool,
}

impl Default for MockMarker {
    fn default() -> Self {
        Self {
            address: Addr::unchecked(DEFAULT_MARKER_ADDRESS),
            coins: coins(DEFAULT_MARKER_HOLDINGS, DEFAULT_MARKER_DENOM),
            account_number: 50,
            sequence: 0,
            manager: "".to_string(),
            permissions: vec![AccessGrant {
                address: MOCK_CONTRACT_ADDR.to_string(),
                permissions: vec![
                    Access::Admin as i32,
                    Access::Burn as i32,
                    Access::Delete as i32,
                    Access::Deposit as i32,
                    Access::Mint as i32,
                    Access::Withdraw as i32,
                ],
            }],
            status: MarkerStatus::Active,
            denom: DEFAULT_MARKER_DENOM.to_string(),
            total_supply: Uint128::new(DEFAULT_MARKER_HOLDINGS),
            marker_type: MarkerType::Coin,
            supply_fixed: true,
        }
    }
}

impl MockMarker {
    pub fn new(supply_fixed: bool, denom_str: String) -> Self {
        Self {
            address: Addr::unchecked(DEFAULT_MARKER_ADDRESS),
            coins: coins(DEFAULT_MARKER_HOLDINGS, denom_str.to_owned()),
            account_number: 50,
            sequence: 0,
            manager: "".to_string(),
            permissions: vec![AccessGrant {
                address: MOCK_CONTRACT_ADDR.to_string(),
                permissions: vec![
                    Access::Admin as i32,
                    Access::Burn as i32,
                    Access::Delete as i32,
                    Access::Deposit as i32,
                    Access::Mint as i32,
                    Access::Withdraw as i32,
                ],
            }],
            status: MarkerStatus::Active,
            denom: denom_str.to_owned(),
            total_supply: Uint128::new(DEFAULT_MARKER_HOLDINGS),
            marker_type: MarkerType::Coin,
            supply_fixed, // 'supply_fixed' passed as argument
        }
    }

    pub fn new_owned_mock_marker<S: Into<String>>(owner_address: S) -> Self {
        Self {
            // permissions: AccessGrant array that always leads with owner permission in test code
            permissions: vec![
                AccessGrant {
                    address: owner_address.into(),
                    permissions: Self::get_default_owner_permissions(),
                },
                AccessGrant {
                    address: "cosmos2contract".to_string(),
                    permissions: vec![Access::Admin as i32, Access::Withdraw as i32],
                },
            ],
            ..Self::default()
        }
    }

    pub fn new_owned_mock_marker_supply_variable<S: Into<String>>(
        owner_address: S,
        denom: Option<S>,
        supply_fixed: bool,
    ) -> Self {
        let default_denom = DEFAULT_MARKER_DENOM;
        let denom_str = denom.map_or(default_denom.into(), Into::into);

        Self {
            // permissions: AccessGrant array that always leads with owner permission in test code
            permissions: vec![
                AccessGrant {
                    address: owner_address.into(),
                    permissions: Self::get_default_owner_permissions(),
                },
                AccessGrant {
                    address: "cosmos2contract".to_string(),
                    permissions: vec![Access::Admin as i32, Access::Withdraw as i32],
                },
            ],
            ..Self::new(supply_fixed, denom_str)
        }
    }

    pub fn new_owned_marker<S: Into<String>>(owner_address: S) -> MockMarker {
        Self::new_owned_mock_marker(owner_address)
    }

    pub fn new_owned_marker_custom<S: Into<String>>(
        owner_address: S,
        denom_str: Option<S>,
        supply_fixed: bool,
    ) -> MockMarker {
        Self::new_owned_mock_marker_supply_variable(owner_address, denom_str, supply_fixed)
            .to_marker()
    }

    pub fn to_marker(self) -> MockMarker {
        MockMarker {
            address: self.address,
            coins: self.coins,
            account_number: self.account_number,
            sequence: self.sequence,
            manager: self.manager,
            permissions: self.permissions,
            status: self.status,
            denom: self.denom,
            total_supply: self.total_supply,
            marker_type: self.marker_type,
            supply_fixed: self.supply_fixed,
        }
    }

    pub fn to_marker_account(self) -> MarkerAccount {
        MarkerAccount {
            base_account: Some(BaseAccount {
                address: self.address.to_string(),
                pub_key: None,
                account_number: self.account_number,
                sequence: self.sequence,
            }),
            manager: self.manager,
            access_control: self.permissions,
            status: self.status.into(),
            denom: self.denom,
            supply: self.total_supply.to_string(),
            marker_type: self.marker_type.into(),
            supply_fixed: self.supply_fixed,
            allow_governance_control: false,
            allow_forced_transfer: false,
            required_attributes: vec![],
        }
    }

    pub fn get_default_owner_permissions() -> Vec<i32> {
        vec![
            Access::Admin as i32,
            Access::Burn as i32,
            Access::Delete as i32,
            Access::Deposit as i32,
            Access::Mint as i32,
            Access::Withdraw as i32,
        ]
    }
}

pub fn decimal(value: u128) -> Decimal {
    Decimal::new(Uint128::new(value))
}
