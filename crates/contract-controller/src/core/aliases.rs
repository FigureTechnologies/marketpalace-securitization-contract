use cosmwasm_std::{Binary, CosmosMsg, Deps, DepsMut, Response, SubMsg};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

use super::error::ContractError;

pub type ProvDeps<'a> = Deps<'a, ProvenanceQuery>;
pub type ProvDepsMut<'a> = DepsMut<'a, ProvenanceQuery>;
pub type ProvTxResponse = Result<Response<ProvenanceMsg>, ContractError>;
pub type ProvQueryResponse = Result<Binary, ContractError>;
pub type ProvMsg = CosmosMsg<ProvenanceMsg>;
pub type ProvSubMsg = SubMsg<ProvenanceMsg>;
