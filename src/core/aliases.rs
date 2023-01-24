use cosmwasm_std::{Binary, Deps, DepsMut, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

use super::error::ContractError;

pub type ProvDeps<'a> = Deps<'a, ProvenanceQuery>;
pub type ProvDepsMut<'a> = DepsMut<'a, ProvenanceQuery>;
pub type ProvTxResponse = Result<Response<ProvenanceMsg>, ContractError>;
pub type ProvQueryResponse = Result<Binary, ContractError>;
