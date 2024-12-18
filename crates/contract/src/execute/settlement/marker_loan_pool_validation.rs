use crate::core::error::ContractError;
use crate::util::provenance_utilities::{
    get_single_marker_coin_holding, marker_has_admin, marker_has_permissions, Marker,
};
use cosmwasm_std::{Addr, DepsMut, Uint128};
use provwasm_std::types::provenance::marker::v1::{Access, MarkerAccount, MarkerStatus};
use result_extensions::ResultExtensions;
use std::str::FromStr;

// New helper function for generating error messages
fn get_contract_error(msg: String) -> Result<(), ContractError> {
    ContractError::InvalidMarker { message: msg }.to_err()
}

/// Validates the marker for loan pool addition or removal.
///
/// This function checks whether the passed marker meets the certain criteria to be added to a loan pool.
/// It ensures that all the necessary permissions on the marker are granted.
/// It checks if the marker is Active and whether it holds an amount of its own denomination (`denom`).
/// The function also checks whether the bank supply is less than what is held in the marker.
///
/// # Arguments
/// * `marker` - A reference to the `Marker` to be validated.
/// * `original_owner_address` - An address, the function checks whether this address has administrative privileges on the marker.
/// * `contract_address` - The address of the contract.
/// * `expected_contract_permissions` - A reference to the permissions that the contract is expected to have on the marker.
/// * `bank_supply` - The total supply held by the bank.
///
/// # Returns
/// * Returns a `Result` with `Ok(())` if the `marker` has the `expected_contract_permissions`, is Active, holds some of its `denom`, and doesn't hold more than what is supplied by the bank.
/// * Returns a `Result` with `Err(ContractError)` if any of the checks fails. Each failure case has a unique error message that describes what went wrong.
pub fn validate_marker_for_loan_pool_add_remove(
    deps: &DepsMut,
    marker: &MarkerAccount,
    original_owner_address: &Addr,
    contract_address: &Addr,
    expected_contract_permissions: &[Access],
    bank_supply: Uint128,
) -> Result<(), ContractError> {
    if !marker_has_admin(marker, original_owner_address) {
        // Update this line
        return get_contract_error(format!(
            "expected sender [{}] to have admin privileges on marker [{}]",
            original_owner_address.as_str(),
            marker.denom,
        ));
    }
    if !marker_has_permissions(marker, contract_address, expected_contract_permissions) {
        return get_contract_error(format!(
            "expected this contract [{}] to have privileges {:?} on marker [{}]",
            contract_address.as_str(),
            expected_contract_permissions,
            marker.denom,
        ));
    }
    // Active check
    if marker.status != MarkerStatus::Active as i32 {
        return get_contract_error(format!(
            "expected marker [{}] to be active, but was in status [{:?}]",
            marker.denom, marker.status,
        ));
    }
    // get denom that this marker holds, where denom == marker denom
    let marker_coin = get_single_marker_coin_holding(deps, marker)?;
    let coin_amount = Uint128::from_str(marker_coin.amount.as_str())?;
    let marker_supply = Uint128::from_str(marker.supply.as_str())?;

    if coin_amount == Uint128::new(0) {
        return get_contract_error(format!(
            "expected marker [{}] to hold at least one of its supply of denom, but it had [{}]",
            marker.denom, marker_coin.amount,
        ));
    }

    // supply fixed then we can trust the marker total_supply
    if marker.supply_fixed {
        // amount held in marker cannot be greater than total supply
        if coin_amount > marker_supply {
            return get_contract_error(format!(
                "expected marker [{}] to be holding all the shares with supply [{}], found [{}]",
                marker.denom, marker_coin.amount, marker.supply,
            ));
        }
    } else {
        // use the bank supply passed in
        // amount held in marker cannot be less than bank supply
        if bank_supply > coin_amount {
            return get_contract_error(format!(
                "expected that marker, [{}] to be holding all the shares with supply, [{}]",
                marker.denom, marker_coin.amount,
            ));
        }
    }

    ().to_ok()
}
