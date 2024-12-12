use cosmwasm_schema::cw_serde;
use crate::core::error::ContractError;
use provwasm_std::types::cosmos::base::v1beta1::Coin;
use cosmwasm_std::{coin, Addr, BankQuery, CosmosMsg, Decimal, DepsMut, Empty, StdError, StdResult, SupplyResponse, Uint128};
use provwasm_std::try_proto_to_cosmwasm_coins;
use provwasm_std::types::cosmos::auth::v1beta1::BaseAccount;
use provwasm_std::types::provenance::marker::v1::{Access, AccessGrant, MarkerAccount, MarkerQuerier, MarkerStatus, MarkerType, MsgActivateRequest, MsgAddAccessRequest, MsgAddMarkerRequest, MsgDeleteAccessRequest, MsgFinalizeRequest, MsgMintRequest, MsgTransferRequest, MsgWithdrawRequest, QueryHoldingRequest, QueryHoldingResponse};
use provwasm_std::types::provenance::msgfees::v1::MsgAssessCustomMsgFeeRequest;
use result_extensions::ResultExtensions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const NHASH: &str = "nhash";

pub fn format_coin_display(coins: &[Coin]) -> String {
    coins
        .iter()
        .map(|coin| format!("{}{}", coin.amount, coin.denom))
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn marker_has_permissions(
    marker: &MarkerAccount,
    address: &Addr,
    expected_permissions: &[Access],
) -> bool {
    marker.access_control.iter().any(|permission| {
        &permission.address == &address.clone().into_string()
            && expected_permissions
                .iter()
                .all(|expected_permission| permission.permissions.contains(&expected_permission.to_i32()))
    })
}

pub trait AccessExt {
    fn to_i32(&self) -> i32;
}

// Implement the trait for the Access enum
impl AccessExt for Access {
    fn to_i32(&self) -> i32 {
        *self as i32
    }
}


pub fn create_marker<S: Into<String>>(
    amount: Uint128,
    denom: S,
    marker_type: MarkerType,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    let coin = Coin {
        amount: amount.to_string(),
        denom: validate_string(denom, "denom")?,
    };

    Ok(MsgAddMarkerRequest {
        amount: Some(coin),
        manager: validate_address(contract_address.clone())?.to_string(),
        from_address: validate_address(contract_address)?.to_string(),
        status: MarkerStatus::Proposed.into(),
        marker_type: marker_type.into(),
        access_list: vec![],
        supply_fixed: false,
        allow_governance_control: false,
        allow_forced_transfer: false,
        required_attributes: vec![],
        usd_cents: 0,
        volume: 0,
        usd_mills: 0,
    }
        .into())
}

pub fn marker_has_admin(marker: &MarkerAccount, admin_address: &Addr) -> bool {
    marker_has_permissions(marker, admin_address, &[Access::Admin])
}

pub struct MockMarker {
    pub address: Addr,
    pub coins: Vec<Coin>,
    pub account_number: u64,
    pub sequence: u64,
    pub manager: String,
    pub permissions: Vec<AccessGrant>,
    pub status: MarkerStatus,
    pub denom: String,
    pub total_supply: Decimal,
    pub marker_type: MarkerType,
    pub supply_fixed: bool,
}

// Retrieves the single coin holding associated with the provided marker.
//
// This function takes a reference to a `Marker` object, iterates through its coins, and filters
// the coins that match the denomination of the marker. It then checks whether there is exactly
// one matching coin. If the marker has a single coin entry with the matching denomination, it
// returns that coin. If there is more than one or none, it returns an error.
//
// # Arguments
//
// * `marker` - A reference to a `Marker` object, representing the marker whose single coin
//   holding is to be retrieved.
//
// # Returns
//
// * `Result<Coin, ContractError>` - Returns a `Coin` object wrapped in an `Ok` variant if
//   the marker contains exactly one coin entry with the given denomination. Returns an `Err`
//   variant with a `ContractError::InvalidMarker` error if the marker does not contain exactly
//   one coin entry with the given denomination.
//
// # Errors
//
// * `ContractError::InvalidMarker` - If the marker does not have exactly one coin entry for
//   the given denomination. The error message includes the marker address, denomination, and
//   current holdings.
//
// # Example
//
// ```ignore
// let marker = get_marker();
// match get_single_marker_coin_holding(&marker) {
//     Ok(coin) => println!("Single coin holding: {}", coin),
//     Err(e) => println!("Error retrieving coin holding: {}", e),
// }
// ```
pub fn get_single_marker_coin_holding(deps: &DepsMut, marker: &MarkerAccount) -> Result<Coin, ContractError> {
    let holding_response: QueryHoldingResponse = deps.querier.query(
        &QueryHoldingRequest {
            id: marker.denom.clone(),
            pagination: None,
        }.into(),
    )?;
    let marker_denom_holdings = holding_response
        .balances
        .iter()
        .cloned()
        .flat_map(|balance| balance.coins)
        .filter(|coin| coin.denom == marker.denom)
        .collect::<Vec<Coin>>();
    let marker_address = get_marker_address(marker.base_account.clone())?;
    // only single coin is permitted
    if marker_denom_holdings.len() != 1 {
        return ContractError::InvalidMarker {
            message: format!(
                "expected marker [{}] to have a single coin entry for denom [{}], but it did not. Holdings: [{}]",
                marker_address.to_string(),
                marker.denom,
                format_coin_display(&marker_denom_holdings),
            )
        }.to_err();
    }
    marker_denom_holdings.first().unwrap().to_owned().to_ok()
}

// pub fn calculate_marker_quote(marker_share_count: u128, quote_per_share: &[Coin]) -> Vec<Coin> {
//     quote_per_share
//         .iter()
//         .map(|c| coin(c.amount.u128() * marker_share_count, &c.denom))
//         .to_owned()
//         .collect::<Vec<Coin>>()
// }

pub fn finalize_marker<S: Into<String>>(denom: S, contract_address: Addr) -> StdResult<CosmosMsg> {
    Ok(MsgFinalizeRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
    }
        .into())
}

// pub fn get_marker_by_denom<H: Into<String>>(
//     denom: H,
//     querier: &MarkerQuerier<Empty>,
// ) -> StdResult<Marker> {
//     get_marker(validate_string(denom, "denom")?, querier)
// }

pub fn get_marker(id: String, querier: &MarkerQuerier<Empty>) -> StdResult<MarkerAccount> {
    let response = querier.marker(id)?;
    if let Some(marker) = response.marker {
        return if let Ok(account) = MarkerAccount::try_from(marker) {
            Ok(account)
        } else {
            Err(StdError::generic_err("unable to type-cast marker account"))
        };
    } else {
        Err(StdError::generic_err("no marker found for id"))
    }
}

pub struct Marker {
    pub marker_account: MarkerAccount,
    pub coins: Vec<Coin>,
}

// pub fn get_marker(id: String, querier: &MarkerQuerier<Empty>) -> StdResult<Marker> {
//     let response = querier.marker(id)?;
//     if let Some(marker) = response.marker {
//         return if let Ok(account) = MarkerAccount::try_from(marker) {
//             let escrow = querier.escrow(account.clone().base_account.unwrap().address)?;
//             Ok(Marker {
//                 marker_account: account.into(),
//                 coins: try_proto_to_cosmwasm_coins(escrow.escrow)?,
//             })
//         } else {
//             Err(StdError::generic_err("unable to type-cast marker account"))
//         };
//     }
//     Err(StdError::generic_err(format!(
//         "no marker found for id: response: {:?}",
//         response
//     )))
// }

// pub fn activate_marker<S: Into<String>>(denom: S) -> StdResult<CosmosMsg> {
//     Ok(create_marker_msg(MarkerMsgParams::ActivateMarker {
//         denom: validate_string(denom, "denom")?,
//     }))
// }
//
pub fn activate_marker<S: Into<String>>(denom: S, contract_address: Addr) -> StdResult<CosmosMsg> {
    Ok(MsgActivateRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
    }
        .into())
}

pub fn transfer_marker_coins<S: Into<String>, H: Into<Addr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
    contract_address: H,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("transfer amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgTransferRequest {
        amount: Some(coin),
        administrator: contract_address.into().to_string(),
        from_address: validate_address(from)?.to_string(),
        to_address: validate_address(to)?.to_string(),
    }
        .into())
}

pub fn mint_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("mint amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };

    Ok(MsgMintRequest {
        amount: Some(coin),
        administrator: validate_address(contract_address)?.to_string(),
    }
        .into())
}

pub fn withdraw_coins<S: Into<String>, H: Into<Addr>>(
    marker_denom: S,
    amount: u128,
    denom: S,
    recipient: H,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("withdraw amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgWithdrawRequest {
        denom: validate_string(marker_denom, "marker_denom")?,
        administrator: validate_address(contract_address)?.to_string(),
        to_address: validate_address(recipient)?.to_string(),
        amount: vec![coin],
    }
        .into())
}

pub fn grant_marker_access<S: Into<String>, H: Into<Addr>>(
    denom: S,
    address: H,
    permissions: Vec<AccessGrant>,
) -> StdResult<CosmosMsg> {
    Ok(MsgAddAccessRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(address)?.to_string(),
        access: permissions,
    }.into())
}

pub fn revoke_marker_access<S: Into<String>, H: Into<Addr> + Clone>(
    denom: S,
    address: H,
) -> StdResult<CosmosMsg> {
    Ok(MsgDeleteAccessRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(address.clone())?.to_string(),
        removed_address: validate_address(address)?.to_string(),
    }.into())
}

/// A helper that ensures string params are non-empty.
pub fn validate_string<S: Into<String>>(input: S, param_name: &str) -> StdResult<String> {
    let s: String = input.into();
    if s.trim().is_empty() {
        let err = format!("{} must not be empty", param_name);
        Err(StdError::generic_err(err))
    } else {
        Ok(s)
    }
}
/// A helper that ensures address params are non-empty.
pub fn validate_address<H: Into<Addr>>(input: H) -> StdResult<Addr> {
    let h: Addr = input.into();
    if h.to_string().trim().is_empty() {
        Err(StdError::generic_err("address must not be empty"))
    } else {
        Ok(h)
    }
}

pub fn release_marker_from_contract<S: Into<String>>(
    marker_denom: S,
    contract_address: &Addr,
    permissions_to_grant: &[AccessGrant],
) -> Result<Vec<CosmosMsg>, ContractError> {
    let marker_denom = marker_denom.into();
    let mut messages = vec![];
    // Restore all permissions that the marker had before it was transferred to the
    // contract.
    // grant_marker_access(&marker_denom, contract_address, permissions_to_grant.to_vec())?;
    for permission in permissions_to_grant {
        messages.push(grant_marker_access(
            &marker_denom,
            Addr::unchecked(permission.address.clone()),
            vec![permission.clone()],
        )?);
    }
    // Remove the contract's ownership of the marker now that it is no longer available for
    // sale / trade.  This message HAS TO COME LAST because the contract will lose its permission
    // to restore the originally-revoked permissions otherwise.
    messages.push(revoke_marker_access(
        &marker_denom,
        contract_address.to_owned(),
    )?);
    messages.to_ok()
}



pub fn assess_custom_fee<S: Into<String>>(
    amount: cosmwasm_std::Coin,
    name: Option<S>,
    from: Addr,
    recipient: Option<Addr>,
) -> Result<cosmwasm_std::CosmosMsg, cosmwasm_std::StdError> {
    let coin = provwasm_std::types::cosmos::base::v1beta1::Coin {
        denom: amount.denom,
        amount: amount.amount.to_string(),
    };

    Ok(MsgAssessCustomMsgFeeRequest {
        name: name.map(|s| s.into()).unwrap_or("".to_string()),
        amount: Some(coin),
        recipient: recipient.unwrap_or(Addr::unchecked("")).to_string(),
        from: validate_address(from)?.to_string(),
        recipient_basis_points: "10000".to_string(),
    }
        .into())
}

pub fn query_total_supply(deps: &DepsMut, denom: String) -> StdResult<Uint128> {
    let request = BankQuery::Supply {
        denom: denom.into(),
    }
    .into();
    let res: SupplyResponse = deps.querier.query(&request)?;
    Ok(res.amount.amount)
}

pub fn get_marker_address(base_account: Option<BaseAccount>) -> Result<String, ContractError> {
    base_account
        .ok_or(ContractError::InvalidMarker {
            message: "Base account is missing".to_string(),
        })
        .map(|account| account.address)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use cosmwasm_std::{coin, coins, Addr, Uint128};
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::types::cosmos::base::v1beta1::Coin;
    use provwasm_std::types::provenance::attribute::v1::AttributeType::String;
    use provwasm_std::types::provenance::marker::v1::{Access, AccessGrant};
    use crate::core::error::ContractError;
    use crate::util::mock_marker::MockMarker;
    use crate::util::provenance_utilities::{format_coin_display, get_single_marker_coin_holding, marker_has_admin, marker_has_permissions, NHASH};

    // use super::*;
    // use crate::util::mock_marker::MockMarker;
    // use cosmwasm_std::{coins, AnyMsg};
    // use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
    // use provwasm_std::types::provenance::marker::v1::{Access, AccessGrant};
    //
    // #[test]
    // fn test_format_coin_display() {
    //     assert_eq!(
    //         "",
    //         format_coin_display(&[]),
    //         "empty display should produce an empty string",
    //     );
    //     assert_eq!(
    //         "150nhash",
    //         format_coin_display(&coins(150, NHASH)),
    //         "single coin display should produce a simple result",
    //     );
    //     assert_eq!(
    //         "12acoin, 13bcoin, 14ccoin",
    //         format_coin_display(&[coin(12, "acoin"), coin(13, "bcoin"), coin(14, "ccoin")]),
    //         "multiple coin display should produce a space-including csv result",
    //     );
    // }
    //
    #[test]
    fn test_marker_has_permissions() {
        let target_address = Addr::unchecked("target_address");
        let marker = MockMarker {
            permissions: vec![AccessGrant {
                address: target_address.to_string(),
                permissions: vec![
                    Access::Admin as i32,
                    Access::Mint as i32,
                    Access::Delete as i32,
                ],
            }],
            ..MockMarker::default()
        }
        .to_marker_account();
        assert!(
            marker_has_permissions(&marker, &target_address, &[]),
            "no permissions passed in with an existing address on the marker should produce a true response",
        );
        assert!(
            marker_has_permissions(&marker, &target_address, &[Access::Admin]),
            "single target permission for correct address should produce a true response",
        );
        assert!(
            marker_has_permissions(&marker, &target_address, &[Access::Admin, Access::Mint, Access::Delete]),
            "multiple target with all values present for correct address should produce a true response",
        );
        assert!(
            !marker_has_permissions(&marker, &Addr::unchecked("not the same address"), &[]),
            "no permissions passed in with an address not found in the marker should produce a false response",
        );
        assert!(
            !marker_has_permissions(&marker, &Addr::unchecked("not the same address"), &[Access::Admin]),
            "single target permission for address not in marker permissions should produce a false response",
        );
        assert!(
            !marker_has_permissions(
                &marker,
                &Addr::unchecked("not the same address"),
                &[
                    Access::Admin,
                    Access::Mint,
                    Access::Delete
                ],
            ),
            "multiple target with bad target address should produce a false response",
        );
    }

    #[test]
    fn test_marker_has_admin() {
        let admin1 = Addr::unchecked("admin1");
        let admin2 = Addr::unchecked("admin2");
        let normie = Addr::unchecked("normie2");
        let missing = Addr::unchecked("missing");
        let marker = MockMarker {
            permissions: vec![
                AccessGrant {
                    address: admin1.to_string(),
                    permissions: vec![Access::Admin as i32],
                },
                AccessGrant {
                    address: admin2.to_string(),
                    permissions: vec![
                        Access::Admin as i32,
                        Access::Mint as i32,
                        Access::Burn as i32,
                        Access::Deposit as i32,
                        Access::Transfer as i32,
                        Access::Delete as i32,
                    ],
                },
                AccessGrant {
                    address: normie.to_string(),
                    permissions: vec![Access::Withdraw as i32, Access::Deposit as i32],
                },
            ],
            ..MockMarker::default()
        }
        .to_marker_account();
        assert!(
            marker_has_admin(&marker, &admin1),
            "the first admin with ONLY admin access type should produce a true response",
        );
        assert!(
            marker_has_admin(&marker, &admin2),
            "the second admin with many access types should produce a true response",
        );
        assert!(
            !marker_has_admin(&marker, &normie),
            "the account without admin access should produce a false response",
        );
        assert!(
            !marker_has_admin(&marker, &missing),
            "the account not present in the marker permissions should produce a false response",
        );
    }

    #[test]
    fn test_get_single_marker_coin_holding() {
        let mut deps = mock_provenance_dependencies();
        let no_denom_marker = MockMarker {
            address: Addr::unchecked("nodenomaddr"),
            denom: "nodenom".to_string(),
            coins: vec![],
            ..MockMarker::default()
        }
        .to_marker_account();
        match get_single_marker_coin_holding(&deps.as_mut(), &no_denom_marker)
            .expect_err("expected an error to occur when a marker had none of its own coin")
        {
            ContractError::InvalidMarker { message } => {
                assert_eq!(
                    message,
                    "expected marker [nodenomaddr] to have a single coin entry for denom [nodenom], but it did not. Holdings: []",
                    "unexpected error message",
                );
            }
            e => panic!("unexpected error encountered: {:?}", e),
        };
        let invalid_coin_marker = MockMarker {
            address: Addr::unchecked("badcoinaddr"),
            denom: "badcoin".to_string(),
            coins: vec![coin(100, "othercoin"), coin(15, "moredifferentcoin")],
            ..MockMarker::default()
        }
        .to_marker_account();
        match get_single_marker_coin_holding(&deps.as_mut(), &invalid_coin_marker).expect_err(
            "expected an error to occur when a marker had other coins, but none of its own",
        ) {
            ContractError::InvalidMarker { message } => {
                assert_eq!(
                    message,
                    "expected marker [badcoinaddr] to have a single coin entry for denom [badcoin], but it did not. Holdings: [100othercoin, 15moredifferentcoin]",
                    "unexpected error message",
                );
            }
            e => panic!("unexpected error encountered: {:?}", e),
        }
        let duplicate_coin_marker = MockMarker {
            address: Addr::unchecked("weirdaddr"),
            denom: "weird".to_string(),
            coins: vec![coin(12, "weird"), coin(15, "weird")],
            ..MockMarker::default()
        }
        .to_marker_account();
        match get_single_marker_coin_holding(&deps.as_mut(), &duplicate_coin_marker).expect_err(
            "expected an error to occur when a marker had more than one entry for its own denom",
        ) {
            ContractError::InvalidMarker { message } => {
                assert_eq!(
                    message,
                    "expected marker [weirdaddr] to have a single coin entry for denom [weird], but it did not. Holdings: [12weird, 15weird]",
                    "unexpected error message",
                );
            }
            e => panic!("unexpected error encountered: {:?}", e),
        };
        let mut good_marker = MockMarker {
            address: Addr::unchecked("goodaddr"),
            denom: "good".to_string(),
            coins: vec![coin(150, "good")],
            ..MockMarker::default()
        }
        .to_marker_account();
        let marker_coin = get_single_marker_coin_holding(&deps.as_mut(), &good_marker).expect(
            "expected a marker containing a single entry of its denom to produce a coin response",
        );
        assert_eq!(
            150.to_string(),
            marker_coin.amount.as_str(),
            "expected the coin's amount to be unaltered",
        );
        assert_eq!(
            "good", marker_coin.denom,
            "expected the coin's denom to be unaltered",
        );
        // good_marker.coins = vec![marker_coin.clone(), coin(10, "bitcoin"), coin(15, NHASH)];
        // let extra_holdings_coin = get_single_marker_coin_holding(&deps.as_mut(), &good_marker).expect("expected a marker containing a single entry of its own denom and some other holdings to produce a coin response");
        // assert_eq!(
        //     marker_coin, extra_holdings_coin,
        //     "the same coin should be produced in similar good scenarios",
        // );
    }
    //
    // #[test]
    // fn test_release_marker_from_contract_produces_correct_output() {
    //     let messages = release_marker_from_contract(
    //         "testdenom",
    //         &Addr::unchecked(MOCK_CONTRACT_ADDR),
    //         &[
    //             AccessGrant {
    //                 address: "asker".to_string(),
    //                 permissions: vec![Access::Admin as i32, Access::Burn as i32],
    //             },
    //             AccessGrant {
    //                 address: "innocent_bystander".to_string(),
    //                 permissions: vec![Access::Withdraw as i32, Access::Transfer as i32],
    //             },
    //         ],
    //     )
    //     .expect("expected a result to be returned for good input");
    //     assert_eq!(
    //         3,
    //         messages.len(),
    //         "the correct number of messages should be produced",
    //     );
    //     messages.into_iter().for_each(|msg| match msg {
    //         CosmosMsg::Any(AnyMsg { params: ProvenanceMsgParams::Marker(MarkerMsgParams::RevokeMarkerAccess { denom, address }), .. }) => {
    //             assert_eq!(
    //                 "testdenom",
    //                 denom,
    //                 "the revocation message should refer to the marker denom",
    //             );
    //             assert_eq!(
    //                 MOCK_CONTRACT_ADDR,
    //                 address.as_str(),
    //                 "the target address for revocation should always be the contract's address",
    //             );
    //         }
    //         CosmosMsg::Any(AnyMsg { params: ProvenanceMsgParams::Marker(MarkerMsgParams::GrantMarkerAccess { denom, address, permissions }), .. }) => {
    //             assert_eq!(
    //                 "testdenom",
    //                 denom,
    //                 "each grant message should refer to the marker's denom",
    //             );
    //             match address.as_str() {
    //                 "asker" => {
    //                     assert_eq!(
    //                         vec![Access::Admin, Access::Burn],
    //                         permissions,
    //                         "expected the asker's permissions to be granted",
    //                     );
    //                 }
    //                 "innocent_bystander" => {
    //                     assert_eq!(
    //                         vec![Access::Withdraw, Access::Transfer],
    //                         permissions,
    //                         "expected the bystander's permissions to be granted",
    //                     );
    //                 }
    //                 addr => panic!("unexpected address encountered in access grants: {}", addr),
    //             };
    //         }
    //         msg => panic!("unexpected message produced: {:?}", msg),
    //     });
    // }
    //
    // #[test]
    // fn test_query_total_supply() {
    //     let amount = coin(12345, "denom");
    //     let mut deps = mock_dependencies_with_balances(&[("alice", &[amount.clone()])]);
    //     // Let's say you have a method to initialize your contract which sets the total supply
    //     // Initialize the contract with a total supply
    //     let total_supply = 12345u128;
    //
    //     // Now, query the total supply using the function you want to test
    //     let result = query_total_supply(&mut deps.as_mut(), "denom").unwrap();
    //
    //     // Assert that the queried total supply matches the expected value
    //     assert_eq!(result.u128(), total_supply);
    // }
}
