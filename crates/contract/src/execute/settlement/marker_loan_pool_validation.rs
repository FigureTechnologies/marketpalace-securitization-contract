use crate::core::error::ContractError;
use crate::execute::settlement::extensions::ResultExtensions;
use crate::util::provenance_utilities::{
    get_single_marker_coin_holding, marker_has_admin, marker_has_permissions,
};
use cosmwasm_std::{Addr, Uint128};
use provwasm_std::{Marker, MarkerAccess, MarkerStatus};

pub fn validate_marker_for_loan_pool_add_remove(
    marker: &Marker,
    original_owner_address: Option<&Addr>,
    contract_address: &Addr,
    expected_contract_permissions: &[MarkerAccess],
    bank_supply: Uint128,
) -> Result<(), ContractError> {
    if let Some(original_owner_address) = original_owner_address {
        if !marker_has_admin(marker, original_owner_address) {
            return ContractError::InvalidMarker {
                message: format!(
                    "expected sender [{}] to have admin privileges on marker [{}]",
                    original_owner_address.as_str(),
                    marker.denom,
                ),
            }
            .to_err();
        }
    }
    if !marker_has_permissions(marker, contract_address, expected_contract_permissions) {
        return ContractError::InvalidMarker {
            message: format!(
                "expected this contract [{}] to have privileges {:?} on marker [{}]",
                contract_address.as_str(),
                expected_contract_permissions,
                marker.denom,
            ),
        }
        .to_err();
    }
    // Active check
    if marker.status != MarkerStatus::Active {
        return ContractError::InvalidMarker {
            message: format!(
                "expected marker [{}] to be active, but was in status [{:?}]",
                marker.denom, marker.status,
            ),
        }
        .to_err();
    }
    // get denom that this marker holds
    let marker_coin = get_single_marker_coin_holding(marker)?;

    if marker_coin.amount.u128() == 0 {
        return ContractError::InvalidMarker {
            message: format!(
                "expected marker [{}] to hold at least one of its supply of denom, but it had [{}]",
                marker.denom,
                marker_coin.amount.u128(),
            ),
        }
        .to_err();
    }
    // supply fixed then we can trust the marker total_supply
    if marker.supply_fixed == true {
        if marker_coin.amount < marker.total_supply.atomics() {
            return ContractError::InvalidMarker {
                message: format!(
                    "expected marker [{}] to be holding all the shares with supply [{}]",
                    marker.denom,
                    marker_coin.amount.u128(),
                ),
            }
            .to_err();
        }
    } else {
        // use the bank supply passed in
        if marker_coin.amount < bank_supply {
            return ContractError::InvalidMarker {
                message: format!(
                    "expected marker [{}] to be holding all the shares with supply [{}]",
                    marker.denom,
                    marker_coin.amount.u128(),
                ),
            }
            .to_err();
        }
    }

    ().to_ok()
}
