use crate::core::error::ContractError;
use crate::execute::settlement::extensions::ResultExtensions;
use cosmwasm_std::{
    coin, Addr, BankQuery, Coin, CosmosMsg, DepsMut, StdResult, SupplyResponse, Uint128,
};
use provwasm_std::{
    grant_marker_access, revoke_marker_access, AccessGrant, Marker, MarkerAccess, ProvenanceMsg,
    ProvenanceQuery,
};

pub const NHASH: &str = "nhash";

pub fn format_coin_display(coins: &[Coin]) -> String {
    coins
        .iter()
        .map(|coin| format!("{}{}", coin.amount.u128(), coin.denom))
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn marker_has_permissions(
    marker: &Marker,
    address: &Addr,
    expected_permissions: &[MarkerAccess],
) -> bool {
    marker.permissions.iter().any(|permission| {
        &permission.address == address
            && expected_permissions
                .iter()
                .all(|expected_permission| permission.permissions.contains(expected_permission))
    })
}

pub fn marker_has_admin(marker: &Marker, admin_address: &Addr) -> bool {
    marker_has_permissions(marker, admin_address, &[MarkerAccess::Admin])
}

/// Retrieves the single coin holding associated with the provided marker.
///
/// This function takes a reference to a `Marker` object, iterates through its coins, and filters
/// the coins that match the denomination of the marker. It then checks whether there is exactly
/// one matching coin. If the marker has a single coin entry with the matching denomination, it
/// returns that coin. If there is more than one or none, it returns an error.
///
/// # Arguments
///
/// * `marker` - A reference to a `Marker` object, representing the marker whose single coin
///   holding is to be retrieved.
///
/// # Returns
///
/// * `Result<Coin, ContractError>` - Returns a `Coin` object wrapped in an `Ok` variant if
///   the marker contains exactly one coin entry with the given denomination. Returns an `Err`
///   variant with a `ContractError::InvalidMarker` error if the marker does not contain exactly
///   one coin entry with the given denomination.
///
/// # Errors
///
/// * `ContractError::InvalidMarker` - If the marker does not have exactly one coin entry for
///   the given denomination. The error message includes the marker address, denomination, and
///   current holdings.
///
/// # Example
///
/// ```ignore
/// let marker = get_marker();
/// match get_single_marker_coin_holding(&marker) {
///     Ok(coin) => println!("Single coin holding: {}", coin),
///     Err(e) => println!("Error retrieving coin holding: {}", e),
/// }
/// ```
pub fn get_single_marker_coin_holding(marker: &Marker) -> Result<Coin, ContractError> {
    let marker_denom_holdings = marker
        .coins
        .iter()
        .cloned()
        .filter(|coin| coin.denom == marker.denom)
        .collect::<Vec<Coin>>();
    // only single coin is permitted
    if marker_denom_holdings.len() != 1 {
        return ContractError::InvalidMarker {
            message: format!(
                "expected marker [{}] to have a single coin entry for denom [{}], but it did not. Holdings: [{}]",
                marker.address.as_str(),
                marker.denom,
                format_coin_display(&marker.coins),
            )
        }.to_err();
    }
    marker_denom_holdings.first().unwrap().to_owned().to_ok()
}

pub fn calculate_marker_quote(marker_share_count: u128, quote_per_share: &[Coin]) -> Vec<Coin> {
    quote_per_share
        .iter()
        .map(|c| coin(c.amount.u128() * marker_share_count, &c.denom))
        .to_owned()
        .collect::<Vec<Coin>>()
}

pub fn release_marker_from_contract<S: Into<String>>(
    marker_denom: S,
    contract_address: &Addr,
    permissions_to_grant: &[AccessGrant],
) -> Result<Vec<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let marker_denom = marker_denom.into();
    let mut messages = vec![];
    // Restore all permissions that the marker had before it was transferred to the
    // contract.
    for permission in permissions_to_grant {
        messages.push(grant_marker_access(
            &marker_denom,
            permission.address.to_owned(),
            permission.permissions.to_owned(),
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

pub fn query_total_supply(deps: &DepsMut<ProvenanceQuery>, denom: &str) -> StdResult<Uint128> {
    let request = BankQuery::Supply {
        denom: denom.into(),
    }
    .into();
    let res: SupplyResponse = deps.querier.query(&request)?;
    Ok(res.amount.amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::mock_marker::MockMarker;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
    use provwasm_mocks::mock_dependencies_with_balances;
    use provwasm_std::{MarkerMsgParams, ProvenanceMsgParams};

    #[test]
    fn test_format_coin_display() {
        assert_eq!(
            "",
            format_coin_display(&[]),
            "empty display should produce an empty string",
        );
        assert_eq!(
            "150nhash",
            format_coin_display(&coins(150, NHASH)),
            "single coin display should produce a simple result",
        );
        assert_eq!(
            "12acoin, 13bcoin, 14ccoin",
            format_coin_display(&[coin(12, "acoin"), coin(13, "bcoin"), coin(14, "ccoin")]),
            "multiple coin display should produce a space-including csv result",
        );
    }

    #[test]
    fn test_marker_has_permissions() {
        let target_address = Addr::unchecked("target_address");
        let marker = MockMarker {
            permissions: vec![AccessGrant {
                address: target_address.clone(),
                permissions: vec![
                    MarkerAccess::Admin,
                    MarkerAccess::Mint,
                    MarkerAccess::Delete,
                ],
            }],
            ..MockMarker::default()
        }
        .to_marker();
        assert!(
            marker_has_permissions(&marker, &target_address, &[]),
            "no permissions passed in with an existing address on the marker should produce a true response",
        );
        assert!(
            marker_has_permissions(&marker, &target_address, &[MarkerAccess::Admin]),
            "single target permission for correct address should produce a true response",
        );
        assert!(
            marker_has_permissions(&marker, &target_address, &[MarkerAccess::Admin, MarkerAccess::Mint, MarkerAccess::Delete]),
            "multiple target with all values present for correct address should produce a true response",
        );
        assert!(
            !marker_has_permissions(&marker, &Addr::unchecked("not the same address"), &[]),
            "no permissions passed in with an address not found in the marker should produce a false response",
        );
        assert!(
            !marker_has_permissions(&marker, &Addr::unchecked("not the same address"), &[MarkerAccess::Admin]),
            "single target permission for address not in marker permissions should produce a false response",
        );
        assert!(
            !marker_has_permissions(
                &marker,
                &Addr::unchecked("not the same address"),
                &[
                    MarkerAccess::Admin,
                    MarkerAccess::Mint,
                    MarkerAccess::Delete
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
                    address: admin1.clone(),
                    permissions: vec![MarkerAccess::Admin],
                },
                AccessGrant {
                    address: admin2.clone(),
                    permissions: vec![
                        MarkerAccess::Admin,
                        MarkerAccess::Mint,
                        MarkerAccess::Burn,
                        MarkerAccess::Deposit,
                        MarkerAccess::Transfer,
                        MarkerAccess::Delete,
                    ],
                },
                AccessGrant {
                    address: normie.clone(),
                    permissions: vec![MarkerAccess::Withdraw, MarkerAccess::Deposit],
                },
            ],
            ..MockMarker::default()
        }
        .to_marker();
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
        let no_denom_marker = MockMarker {
            address: Addr::unchecked("nodenomaddr"),
            denom: "nodenom".to_string(),
            coins: vec![],
            ..MockMarker::default()
        }
        .to_marker();
        match get_single_marker_coin_holding(&no_denom_marker)
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
        .to_marker();
        match get_single_marker_coin_holding(&invalid_coin_marker).expect_err(
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
        .to_marker();
        match get_single_marker_coin_holding(&duplicate_coin_marker).expect_err(
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
        .to_marker();
        let marker_coin = get_single_marker_coin_holding(&good_marker).expect(
            "expected a marker containing a single entry of its denom to produce a coin response",
        );
        assert_eq!(
            150,
            marker_coin.amount.u128(),
            "expected the coin's amount to be unaltered",
        );
        assert_eq!(
            "good", marker_coin.denom,
            "expected the coin's denom to be unaltered",
        );
        good_marker.coins = vec![marker_coin.clone(), coin(10, "bitcoin"), coin(15, NHASH)];
        let extra_holdings_coin = get_single_marker_coin_holding(&good_marker).expect("expected a marker containing a single entry of its own denom and some other holdings to produce a coin response");
        assert_eq!(
            marker_coin, extra_holdings_coin,
            "the same coin should be produced in similar good scenarios",
        );
    }

    #[test]
    fn test_release_marker_from_contract_produces_correct_output() {
        let messages = release_marker_from_contract(
            "testdenom",
            &Addr::unchecked(MOCK_CONTRACT_ADDR),
            &[
                AccessGrant {
                    address: Addr::unchecked("asker"),
                    permissions: vec![MarkerAccess::Admin, MarkerAccess::Burn],
                },
                AccessGrant {
                    address: Addr::unchecked("innocent_bystander"),
                    permissions: vec![MarkerAccess::Withdraw, MarkerAccess::Transfer],
                },
            ],
        )
        .expect("expected a result to be returned for good input");
        assert_eq!(
            3,
            messages.len(),
            "the correct number of messages should be produced",
        );
        messages.into_iter().for_each(|msg| match msg {
            CosmosMsg::Custom(ProvenanceMsg { params: ProvenanceMsgParams::Marker(MarkerMsgParams::RevokeMarkerAccess { denom, address }), .. }) => {
                assert_eq!(
                    "testdenom",
                    denom,
                    "the revocation message should refer to the marker denom",
                );
                assert_eq!(
                    MOCK_CONTRACT_ADDR,
                    address.as_str(),
                    "the target address for revocation should always be the contract's address",
                );
            }
            CosmosMsg::Custom(ProvenanceMsg { params: ProvenanceMsgParams::Marker(MarkerMsgParams::GrantMarkerAccess { denom, address, permissions }), .. }) => {
                assert_eq!(
                    "testdenom",
                    denom,
                    "each grant message should refer to the marker's denom",
                );
                match address.as_str() {
                    "asker" => {
                        assert_eq!(
                            vec![MarkerAccess::Admin, MarkerAccess::Burn],
                            permissions,
                            "expected the asker's permissions to be granted",
                        );
                    }
                    "innocent_bystander" => {
                        assert_eq!(
                            vec![MarkerAccess::Withdraw, MarkerAccess::Transfer],
                            permissions,
                            "expected the bystander's permissions to be granted",
                        );
                    }
                    addr => panic!("unexpected address encountered in access grants: {}", addr),
                };
            }
            msg => panic!("unexpected message produced: {:?}", msg),
        });
    }

    #[test]
    fn test_query_total_supply() {
        let amount = coin(12345, "denom");
        let mut deps = mock_dependencies_with_balances(&[("alice", &[amount.clone()])]);
        // Let's say you have a method to initialize your contract which sets the total supply
        // Initialize the contract with a total supply
        let total_supply = 12345u128;

        // Now, query the total supply using the function you want to test
        let result = query_total_supply(&mut deps.as_mut(), "denom").unwrap();

        // Assert that the queried total supply matches the expected value
        assert_eq!(result.u128(), total_supply);
    }
}
