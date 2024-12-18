use cosmwasm_std::{Binary, CosmosMsg, Deps, DepsMut, Response, SubMsg};

use super::error::ContractError;

pub type ProvDeps<'a> = Deps<'a>;
pub type ProvDepsMut<'a> = DepsMut<'a>;
pub type ProvTxResponse = Result<Response, ContractError>;
pub type ProvQueryResponse = Result<Binary, ContractError>;
pub type ProvMsg = CosmosMsg;
pub type ProvSubMsg = SubMsg;
