use cosmwasm_std::{Addr, Attribute, CosmosMsg, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use provwasm_std::{
    activate_marker, assess_custom_fee, create_marker, finalize_marker, grant_marker_access,
    MarkerAccess, MarkerType, ProvenanceMsg,
};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::InstantiateMsg,
    },
    storage::{
        remaining_securities,
        securities::{self},
        state::{self, State},
    },
    util::to,
};

pub fn handle(
    deps: ProvDepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::new(msg.gp, msg.capital_denom, msg.settlement_time);
    let mut response = Response::new();
    state::set(deps.storage, &state)?;

    // Create the markers
    let mut messages: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();
    for security in &msg.securities {
        let investment_name =
            to::security_to_investment_name(&security.name, &env.contract.address);
        let mut investment_marker =
            new_active_marker(env.contract.address.clone(), &investment_name, 0)?;
        messages.append(&mut investment_marker);
        securities::set(deps.storage, security)?;
        remaining_securities::set(deps.storage, security.name.clone(), security.amount.u128())?;
    }

    response = response
        .add_messages(messages)
        .add_attribute("action", "init");

    if let Some(fee) = msg.fee {
        let recipient = match fee.recipient.clone() {
            Some(addr) => addr.to_string(),
            None => "msgfees_module".to_string(),
        };
        response = response.add_message(assess_custom_fee(
            fee.amount.to_owned(),
            Some("security_creation_fee"),
            env.contract.address,
            fee.recipient,
        )?);
        response = response.add_attributes(vec![
            Attribute::new("fee_recipient", recipient),
            Attribute::new("fee_amount", format!("{:?}", fee.amount)),
        ]);
    }

    Ok(response)
}

fn new_active_marker(
    owner: Addr,
    denom: &String,
    amount: u128,
) -> StdResult<Vec<CosmosMsg<ProvenanceMsg>>> {
    let permissions = vec![
        MarkerAccess::Admin,
        MarkerAccess::Mint,
        MarkerAccess::Burn,
        MarkerAccess::Withdraw,
        MarkerAccess::Transfer,
    ];
    Ok(vec![
        create_marker(amount, denom.clone(), MarkerType::Restricted)?,
        grant_marker_access(denom, owner, permissions)?,
        finalize_marker(denom)?,
        activate_marker(denom)?,
    ])
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        Addr, Coin, StdError, Uint64,
    };
    use cosmwasm_std::{Attribute, Uint128};
    use cw2::get_contract_version;
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{
        activate_marker, create_marker, finalize_marker, grant_marker_access, MarkerAccess,
        MarkerType,
    };

    use crate::storage::state::{self};
    use crate::{
        contract::instantiate,
        core::{
            constants::{CONTRACT_NAME, CONTRACT_VERSION},
            msg::InstantiateMsg,
            security::{Security, TrancheSecurity},
        },
    };
    use crate::{
        core::fee::Fee,
        storage::securities::{self},
    };
    use crate::{instantiate::handler::new_active_marker, storage::remaining_securities};

    #[test]
    fn test_new_active_marker_creates_and_activates_marker() {
        let address = Addr::unchecked("address");
        let denom = "denom".to_string();
        let amount = 1000;
        let permissions = vec![
            MarkerAccess::Admin,
            MarkerAccess::Mint,
            MarkerAccess::Burn,
            MarkerAccess::Withdraw,
            MarkerAccess::Transfer,
        ];

        let messages = new_active_marker(address.clone(), &denom, amount).unwrap();
        assert_eq!(4, messages.len());
        assert_eq!(
            create_marker(amount, &denom, MarkerType::Restricted).unwrap(),
            messages[0]
        );
        assert_eq!(
            grant_marker_access(&denom, address.clone(), permissions,).unwrap(),
            messages[1]
        );
        assert_eq!(finalize_marker(&denom).unwrap(), messages[2]);
        assert_eq!(activate_marker(&denom).unwrap(), messages[3]);
    }

    #[test]
    fn test_new_active_marker_throws_errors_on_invalid_marker_txs() {
        let bad_addr =
            new_active_marker(Addr::unchecked(""), &"mycustomdenom".to_string(), 1000).unwrap_err();
        let expected = StdError::generic_err("address must not be empty");
        assert_eq!(expected, bad_addr);

        let bad_denom =
            new_active_marker(Addr::unchecked("address"), &"".to_string(), 1000).unwrap_err();
        let expected = StdError::generic_err("denom must not be empty");
        assert_eq!(expected, bad_denom);
    }

    #[test]
    fn test_with_valid_data() {
        // create valid init data
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("admin", &[]);
        const DEFAULT_GP: &str = "gp";
        const DEFAULT_TIME: Option<Uint64> = None;
        const DEFAULT_CAPITAL_DENOM: &str = "denom";
        let securities = vec![
            Security {
                name: "Tranche 1".to_string(),
                amount: Uint128::new(1000),
                minimum_amount: Uint128::new(100),
                price_per_unit: Coin::new(100, "denom"),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            },
            Security {
                name: "Tranche 2".to_string(),
                amount: Uint128::new(1000),
                minimum_amount: Uint128::new(100),
                price_per_unit: Coin::new(100, "denom"),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            },
        ];
        let init_msg = InstantiateMsg {
            gp: Addr::unchecked(DEFAULT_GP),
            securities: securities.clone(),
            capital_denom: DEFAULT_CAPITAL_DENOM.to_string(),
            settlement_time: DEFAULT_TIME,
            fee: None,
        };

        // initialize
        let init_response = instantiate(deps.as_mut(), mock_env(), info, init_msg.clone());

        // Check the messages
        match init_response {
            Ok(res) => {
                assert_eq!(8, res.messages.len());
                // We probably want to check the type of messages
                assert_eq!(1, res.attributes.len());
                assert_eq!(Attribute::new("action", "init"), res.attributes[0])
            }
            Err(error) => panic!("unable to initialize contract {}", error),
        };

        // Check the contract version
        let contract_version = get_contract_version(&deps.storage).unwrap();
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(CONTRACT_NAME, contract_version.contract);

        // Check the STATE
        let state = state::get(&deps.storage).unwrap();
        assert_eq!(DEFAULT_CAPITAL_DENOM.to_string(), state.capital_denom);
        assert_eq!(Addr::unchecked(DEFAULT_GP), state.gp);
        assert_eq!(DEFAULT_TIME, state.settlement_time);

        // Check the SECURITIES_MAP
        for security in securities {
            let saved = securities::get(&deps.storage, security.name.clone()).unwrap();
            assert_eq!(security, saved);
            let remaining =
                remaining_securities::get(&deps.storage, security.name.clone()).unwrap();
            assert_eq!(security.amount, Uint128::new(remaining));
        }
    }

    #[test]
    fn test_with_valid_data_and_fee() {
        // create valid init data
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("admin", &[]);
        const DEFAULT_GP: &str = "gp";
        const DEFAULT_CAPITAL_DENOM: &str = "denom";
        const DEFAULT_TIME: Option<Uint64> = None;
        let securities = vec![
            Security {
                name: "Tranche 1".to_string(),
                amount: Uint128::new(1000),
                minimum_amount: Uint128::new(100),
                price_per_unit: Coin::new(100, "denom"),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            },
            Security {
                name: "Tranche 2".to_string(),
                amount: Uint128::new(1000),
                minimum_amount: Uint128::new(100),
                price_per_unit: Coin::new(100, "denom"),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            },
        ];
        let init_msg = InstantiateMsg {
            gp: Addr::unchecked(DEFAULT_GP),
            securities: securities.clone(),
            capital_denom: DEFAULT_CAPITAL_DENOM.to_string(),
            settlement_time: DEFAULT_TIME,
            fee: Some(Fee {
                recipient: Some(Addr::unchecked("recipient")),
                amount: Coin::new(100, "nhash"),
            }),
        };

        // initialize
        let init_response = instantiate(deps.as_mut(), mock_env(), info, init_msg.clone());

        // Check the messages
        match init_response {
            Ok(res) => {
                assert_eq!(9, res.messages.len());
                assert_eq!(
                    vec![
                        Attribute::new("action", "init"),
                        Attribute::new("fee_recipient", "recipient"),
                        Attribute::new("fee_amount", format!("{:?}", Coin::new(100, "nhash")))
                    ],
                    res.attributes
                )
            }
            Err(error) => panic!("unable to initialize contract {}", error),
        };

        // Check the contract version
        let contract_version = get_contract_version(&deps.storage).unwrap();
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(CONTRACT_NAME, contract_version.contract);

        // Check the STATE
        let state = state::get(&deps.storage).unwrap();
        assert_eq!(DEFAULT_CAPITAL_DENOM.to_string(), state.capital_denom);
        assert_eq!(Addr::unchecked(DEFAULT_GP), state.gp);
        assert_eq!(DEFAULT_TIME, state.settlement_time);

        // Check the SECURITIES_MAP
        for security in securities {
            let saved = securities::get(&deps.storage, security.name.clone()).unwrap();
            assert_eq!(security, saved);
            let remaining =
                remaining_securities::get(&deps.storage, security.name.clone()).unwrap();
            assert_eq!(security.amount, Uint128::new(remaining));
        }
    }

    #[test]
    fn test_with_valid_data_and_provenance_fee() {
        // create valid init data
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("admin", &[]);
        const DEFAULT_GP: &str = "gp";
        const DEFAULT_CAPITAL_DENOM: &str = "denom";
        const DEFAULT_TIME: Option<Uint64> = None;
        let securities = vec![
            Security {
                name: "Tranche 1".to_string(),
                amount: Uint128::new(1000),
                minimum_amount: Uint128::new(100),
                price_per_unit: Coin::new(100, "denom"),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            },
            Security {
                name: "Tranche 2".to_string(),
                amount: Uint128::new(1000),
                minimum_amount: Uint128::new(100),
                price_per_unit: Coin::new(100, "denom"),
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            },
        ];
        let init_msg = InstantiateMsg {
            gp: Addr::unchecked(DEFAULT_GP),
            securities: securities.clone(),
            capital_denom: DEFAULT_CAPITAL_DENOM.to_string(),
            settlement_time: DEFAULT_TIME,
            fee: Some(Fee {
                recipient: None,
                amount: Coin::new(100, "nhash"),
            }),
        };

        // initialize
        let init_response = instantiate(deps.as_mut(), mock_env(), info, init_msg.clone());

        // Check the messages
        match init_response {
            Ok(res) => {
                assert_eq!(9, res.messages.len());
                assert_eq!(
                    vec![
                        Attribute::new("action", "init"),
                        Attribute::new("fee_recipient", "msgfees_module"),
                        Attribute::new("fee_amount", format!("{:?}", Coin::new(100, "nhash")))
                    ],
                    res.attributes
                )
            }
            Err(error) => panic!("unable to initialize contract {}", error),
        };

        // Check the contract version
        let contract_version = get_contract_version(&deps.storage).unwrap();
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(CONTRACT_NAME, contract_version.contract);

        // Check the STATE
        let state = state::get(&deps.storage).unwrap();
        assert_eq!(DEFAULT_CAPITAL_DENOM.to_string(), state.capital_denom);
        assert_eq!(Addr::unchecked(DEFAULT_GP), state.gp);
        assert_eq!(DEFAULT_TIME, state.settlement_time);

        // Check the SECURITIES_MAP
        for security in securities {
            let saved = securities::get(&deps.storage, security.name.clone()).unwrap();
            assert_eq!(security, saved);
            let remaining =
                remaining_securities::get(&deps.storage, security.name.clone()).unwrap();
            assert_eq!(security.amount, Uint128::new(remaining));
        }
    }

    #[test]
    fn test_with_invalid_data() {
        // create valid init data
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("admin", &[]);
        const DEFAULT_GP: &str = "gp";
        const DEFAULT_CAPITAL_DENOM: &str = "denom";
        const DEFAULT_TIME: Option<Uint64> = None;
        let securities = vec![];
        let init_msg = InstantiateMsg {
            gp: Addr::unchecked(DEFAULT_GP),
            securities: securities.clone(),
            capital_denom: DEFAULT_CAPITAL_DENOM.to_string(),
            settlement_time: DEFAULT_TIME,
            fee: None,
        };

        // initialize
        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg.clone());
        res.expect_err("expected error in invalid instantiate");
    }
}
