use crate::core::error::ContractError;
use crate::execute::settlement::extensions::ResultExtensions;
use crate::util::provenance_utilities::{
    get_single_marker_coin_holding, marker_has_admin, marker_has_permissions,
};
use cosmwasm_std::{Addr, Uint128};
use provwasm_std::{Marker, MarkerAccess, MarkerStatus};


// New helper function for generating error messages
fn get_contract_error(msg: String) -> Result<(), ContractError> {
    ContractError::InvalidMarker {
        message: msg,
    }
        .to_err()
}


pub fn validate_marker_for_loan_pool_add_remove(
    marker: &Marker,
    original_owner_address: Option<&Addr>,
    contract_address: &Addr,
    expected_contract_permissions: &[MarkerAccess],
    bank_supply: Uint128,
) -> Result<(), ContractError> {
    if let Some(original_owner_address) = original_owner_address {
        if !marker_has_admin(marker, original_owner_address) {
            return get_contract_error(format!(
                "expected sender [{}] to have admin privileges on marker [{}]",
                original_owner_address.as_str(),
                marker.denom,
            ));
        }
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
    if marker.status != MarkerStatus::Active {
        return get_contract_error(format!(
            "expected marker [{}] to be active, but was in status [{:?}]",
            marker.denom, marker.status,
        ));
    }
    // get denom that this marker holds
    let marker_coin = get_single_marker_coin_holding(marker)?;

    if marker_coin.amount.u128() == 0 {
        return get_contract_error(
            format!(
                "expected marker [{}] to hold at least one of its supply of denom, but it had [{}]",
                marker.denom,
                marker_coin.amount.u128(),
            ));
    }
    // supply fixed then we can trust the marker total_supply
    if marker.supply_fixed {
         // amount held in marker cannot be greater than total supply
        if marker_coin.amount > marker.total_supply.atomics() {
            return get_contract_error(format!(
                "expected marker [{}] to be holding all the shares with supply [{}]",
                marker.denom,
                marker_coin.amount.u128(),
            ));
        }
    } else {
        // use the bank supply passed in
        // amount held in marker cannot be less than bank supply
        if bank_supply > marker_coin.amount  {
            return get_contract_error(format!(
                "expected that marker, [{}] to be holding all the shares with supply, [{}]",
                marker.denom,
                marker_coin.amount.u128(),
            ));
        }
    }

    ().to_ok()
}
