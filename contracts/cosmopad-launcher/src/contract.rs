use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, msg::InstantiateMsg, state::BondingCurveState};

/// Placeholder entry point. V0.1 intentionally does not move ATOM or tokens.
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    // TODO: validate and persist config/state after the math layer is reviewed.
    Ok(Response::new())
}

pub fn initialize_curve(atom_reserve: u128, token_reserve: u128) -> Result<BondingCurveState, ContractError> {
    BondingCurveState::new(atom_reserve.into(), token_reserve.into())
}
