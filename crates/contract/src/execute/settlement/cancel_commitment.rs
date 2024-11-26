use cosmwasm_std::{Addr, BankMsg, Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvMsg, ProvTxResponse},
        error::ContractError,
    },
    storage::{available_capital, commits, paid_in_capital, remaining_securities, state},
};

use super::commitment::CommitmentState;

pub fn handle(deps: ProvDepsMut, _env: Env, sender: Addr, commitment_lp: Addr) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    let mut response = Response::default();
    if sender != state.gp && sender != commitment_lp {
        return Err(ContractError::Unauthorized {});
    }

    // It cannot be in settled
    let commit = commits::get(deps.storage, commitment_lp.clone())?;
    if commit.state == CommitmentState::SETTLED {
        return Err(ContractError::AlreadySettled {});
    }

    let refund_messages = refund_lp(deps, commitment_lp.clone())?;
    if !refund_messages.is_empty() {
        response = response.add_messages(refund_messages);
    }

    Ok(response
        .add_attribute("action", "cancel_commitment")
        .add_attribute("sender", sender)
        .add_attribute("canceled_lp", commitment_lp))
}

fn refund_lp(deps: ProvDepsMut, commitment_lp: Addr) -> Result<Vec<ProvMsg>, ContractError> {
    let mut messages = vec![];

    commits::remove(deps.storage, commitment_lp.clone());

    let paid_in_capital = paid_in_capital::get(deps.storage, commitment_lp.clone());
    paid_in_capital::remove(deps.storage, commitment_lp.clone());
    if !paid_in_capital.is_empty() && available_capital::has_lp(deps.storage, commitment_lp.clone())
    {
        // This is what we end up sending back to the lp
        let removed_capital =
            available_capital::remove_capital(deps.storage, commitment_lp.clone())?;

        if !removed_capital.amount.is_zero() {
            messages.push(ProvMsg::Bank(BankMsg::Send {
                to_address: commitment_lp.to_string(),
                amount: vec![removed_capital],
            }));
        }
    }

    for security in paid_in_capital {
        remaining_securities::add(deps.storage, security.name, security.amount.u128())?;
    }

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, BankMsg, Coin, SubMsg};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{aliases::ProvMsg, error::ContractError},
        storage::{
            available_capital, commits, paid_in_capital, remaining_securities,
            state::{self, State},
        },
        util::testing::{create_testing_commitments, instantiate_contract},
    };

    #[test]
    fn test_handle_should_fail_when_sender_is_neither_gp_nor_owner() {
        let gp = Addr::unchecked("gp");
        let sender = Addr::unchecked("lp1");
        let commitment_lp = Addr::unchecked("lp2");
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        state::set(
            deps.as_mut().storage,
            &State::new(gp, "denom".to_string(), None),
        )
        .unwrap();

        let error = super::handle(deps.as_mut(), env, sender, commitment_lp).unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_handle_should_fail_when_settled() {
        let sender = Addr::unchecked("lp7");
        let commitment_lp = Addr::unchecked("lp7");
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();

        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);

        let error = super::handle(deps.as_mut(), env, sender, commitment_lp).unwrap_err();
        assert_eq!(
            ContractError::AlreadySettled {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_handle_should_have_messages_and_attributes() {
        let sender = Addr::unchecked("gp");
        let commitment_lp = Addr::unchecked("lp2");
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let removed_capital = Coin::new(10000, "denom");

        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);

        let res = super::handle(deps.as_mut(), env, sender.clone(), commitment_lp.clone()).unwrap();
        assert_eq!(0, res.events.len());
        assert_eq!(
            vec![
                Attribute::new("action", "cancel_commitment"),
                Attribute::new("sender", sender.to_string()),
                Attribute::new("canceled_lp", commitment_lp.to_string())
            ],
            res.attributes
        );
        assert_eq!(
            vec![SubMsg::new(ProvMsg::Bank(BankMsg::Send {
                to_address: commitment_lp.to_string(),
                amount: vec![removed_capital],
            }))],
            res.messages
        );
    }

    #[test]
    fn test_refund_should_ignore_invalid_lp() {
        let commitment_lp = Addr::unchecked("lp10");
        let mut deps = mock_provenance_dependencies();

        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);

        let res = super::refund_lp(deps.as_mut(), commitment_lp.clone()).unwrap();
        assert_eq!(0, res.len());
    }

    #[test]
    fn test_refund_should_handle_proposed_commit() {
        let commitment_lp = Addr::unchecked("lp4");
        let mut deps = mock_provenance_dependencies();

        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);

        let res = super::refund_lp(deps.as_mut(), commitment_lp.clone()).unwrap();
        assert_eq!(0, res.len());
        assert_eq!(false, commits::exists(deps.as_ref().storage, commitment_lp))
    }

    #[test]
    fn test_refund_should_handle_accepted_commit() {
        let commitment_lp = Addr::unchecked("lp2");
        let mut deps = mock_provenance_dependencies();

        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);

        let refund_messages = super::refund_lp(deps.as_mut(), commitment_lp.clone()).unwrap();
        assert_eq!(
            false,
            commits::exists(deps.as_ref().storage, commitment_lp.clone())
        );
        assert_eq!(
            false,
            available_capital::has_lp(deps.as_ref().storage, commitment_lp.clone())
        );
        assert_eq!(
            false,
            paid_in_capital::has_lp(deps.as_ref().storage, commitment_lp.clone())
        );
        assert_eq!(
            650,
            remaining_securities::get(&deps.storage, "Security1".to_string()).unwrap()
        );
        assert_eq!(
            650,
            remaining_securities::get(&deps.storage, "Security2".to_string()).unwrap()
        );
        assert_eq!(
            vec![ProvMsg::Bank(BankMsg::Send {
                to_address: commitment_lp.to_string(),
                amount: vec![Coin::new(10000, "denom")],
            })],
            refund_messages
        );
    }

    #[test]
    fn test_refund_should_handle_accepted_commit_with_no_deposit() {
        let commitment_lp = Addr::unchecked("lp3");
        let mut deps = mock_provenance_dependencies();

        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);

        let refund_messages = super::refund_lp(deps.as_mut(), commitment_lp.clone()).unwrap();
        assert_eq!(
            false,
            commits::exists(deps.as_ref().storage, commitment_lp.clone())
        );
        assert_eq!(
            false,
            available_capital::has_lp(deps.as_ref().storage, commitment_lp.clone())
        );
        assert_eq!(
            false,
            paid_in_capital::has_lp(deps.as_ref().storage, commitment_lp.clone())
        );
        assert_eq!(
            600,
            remaining_securities::get(&deps.storage, "Security1".to_string()).unwrap()
        );
        assert_eq!(
            600,
            remaining_securities::get(&deps.storage, "Security2".to_string()).unwrap()
        );
        assert_eq!(0, refund_messages.len());
    }
}
