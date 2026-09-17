use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    error::ContractError,
    math::{calculate_buy, calculate_sell, enforce_minimum, initial_price},
    msg::{ConfigResponse, CurveStateResponse, EstimateResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{validate_launch_config, BondingCurveState, LaunchConfig, CONFIG, CURVE},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let owner = deps.api.addr_validate(&msg.owner).map_err(|_| {
        ContractError::InvalidConfiguration("owner is not a valid address".into())
    })?;

    let config = LaunchConfig {
        owner,
        atom_denom: msg.atom_denom,
        token_name: msg.token_name,
        token_symbol: msg.token_symbol,
        token_decimals: msg.token_decimals,
        fee_bps: msg.fee_bps,
        min_buy_amount: msg.min_buy_amount,
        min_sell_amount: msg.min_sell_amount,
    };
    validate_launch_config(&config)?;

    let curve = BondingCurveState::new(msg.virtual_atom_reserve, msg.virtual_token_reserve)?;
    CONFIG.save(deps.storage, &config)?;
    CURVE.save(deps.storage, &curve)?;

    Ok(Response::new().add_attribute("action", "instantiate").add_attribute("version", "v0.1"))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Placeholder => Err(ContractError::NotImplemented(
            "trading and token transfers are disabled in V0.1".into(),
        )),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::CurveState {} => to_json_binary(&query_curve(deps)?),
        QueryMsg::InitialPrice {} => to_json_binary(&initial_price_response(deps)?),
        QueryMsg::BuyEstimate { atom_in, min_tokens_out } => {
            to_json_binary(&buy_estimate(deps, atom_in, min_tokens_out)?)
        }
        QueryMsg::SellEstimate { token_in, min_atom_out } => {
            to_json_binary(&sell_estimate(deps, token_in, min_atom_out)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        atom_denom: config.atom_denom,
        token_name: config.token_name,
        token_symbol: config.token_symbol,
        token_decimals: config.token_decimals,
        fee_bps: config.fee_bps,
        min_buy_amount: config.min_buy_amount,
        min_sell_amount: config.min_sell_amount,
    })
}

fn query_curve(deps: Deps) -> StdResult<CurveStateResponse> {
    let curve = CURVE.load(deps.storage)?;
    Ok(CurveStateResponse {
        atom_reserve: curve.atom_reserve,
        token_reserve: curve.token_reserve,
        k: curve.k.to_string(),
    })
}

fn initial_price_response(deps: Deps) -> StdResult<Uint128Response> {
    let curve = CURVE.load(deps.storage)?;
    let price = initial_price(curve.atom_reserve, curve.token_reserve)
        .map_err(|err| cosmwasm_std::StdError::generic_err(err.to_string()))?;
    Ok(Uint128Response { value: price })
}

fn buy_estimate(deps: Deps, atom_in: Uint128, minimum: Uint128) -> StdResult<EstimateResponse> {
    let curve = CURVE.load(deps.storage)?;
    let output = calculate_buy(atom_in, curve.atom_reserve, curve.token_reserve)
        .map_err(|err| cosmwasm_std::StdError::generic_err(err.to_string()))?;
    enforce_minimum(output, minimum)
        .map_err(|err| cosmwasm_std::StdError::generic_err(err.to_string()))?;
    Ok(EstimateResponse { input: atom_in, output, minimum_output: minimum })
}

fn sell_estimate(deps: Deps, token_in: Uint128, minimum: Uint128) -> StdResult<EstimateResponse> {
    let curve = CURVE.load(deps.storage)?;
    let output = calculate_sell(token_in, curve.atom_reserve, curve.token_reserve)
        .map_err(|err| cosmwasm_std::StdError::generic_err(err.to_string()))?;
    enforce_minimum(output, minimum)
        .map_err(|err| cosmwasm_std::StdError::generic_err(err.to_string()))?;
    Ok(EstimateResponse { input: token_in, output, minimum_output: minimum })
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, schemars::JsonSchema)]
pub struct Uint128Response { pub value: Uint128 }
