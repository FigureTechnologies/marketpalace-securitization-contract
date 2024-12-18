use cosmwasm_std::{Binary, CosmosMsg, Deps, DepsMut, Response};

use super::error::ContractError;

pub type ProvDeps<'a> = Deps<'a>;
pub type ProvDepsMut<'a> = DepsMut<'a>;
pub type ProvResponse = Response;
pub type ProvTxResponse = Result<ProvResponse, ContractError>;
pub type ProvQueryResponse = Result<Binary, ContractError>;
pub type ProvMsg = CosmosMsg;
