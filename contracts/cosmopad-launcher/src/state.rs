use cosmwasm_std::{Uint128, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LaunchConfig {
    pub owner: String,
    pub atom_denom: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimals: u8,
    pub fee_bps: u16,
    pub min_buy_amount: Uint128,
    pub min_sell_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BondingCurveState {
    pub atom_reserve: Uint128,
    pub token_reserve: Uint128,
    /// k = atom_reserve * token_reserve. Uint256 gives extra multiplication space.
    pub k: Uint256,
}

impl BondingCurveState {
    pub fn new(atom_reserve: Uint128, token_reserve: Uint128) -> Result<Self, ContractError> {
        if atom_reserve.is_zero() || token_reserve.is_zero() {
            return Err(ContractError::InvalidInput(
                "virtual reserves must be greater than zero".into(),
            ));
        }
        let k = Uint256::from(atom_reserve)
            .checked_mul(Uint256::from(token_reserve))
            .map_err(|_| ContractError::MathOverflow)?;
        Ok(Self { atom_reserve, token_reserve, k })
    }
}

pub fn validate_launch_config(config: &LaunchConfig) -> Result<(), ContractError> {
    if config.owner.trim().is_empty() { return Err(ContractError::InvalidConfiguration("owner is empty".into())); }
    if config.atom_denom.trim().is_empty() { return Err(ContractError::InvalidConfiguration("atom_denom is empty".into())); }
    if config.token_name.trim().is_empty() { return Err(ContractError::InvalidConfiguration("token_name is empty".into())); }
    if config.token_symbol.trim().is_empty() { return Err(ContractError::InvalidConfiguration("token_symbol is empty".into())); }
    if config.token_decimals > 18 { return Err(ContractError::InvalidConfiguration("token_decimals must be <= 18".into())); }
    if config.fee_bps > 10_000 { return Err(ContractError::InvalidConfiguration("fee_bps must be <= 10000".into())); }
    if config.min_buy_amount.is_zero() || config.min_sell_amount.is_zero() {
        return Err(ContractError::InvalidConfiguration("minimum amounts must be greater than zero".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_config() -> LaunchConfig {
        LaunchConfig { owner: "alice".into(), atom_denom: "uatom".into(), token_name: "CosmoPad".into(), token_symbol: "CMP".into(), token_decimals: 6, fee_bps: 250, min_buy_amount: Uint128::new(1), min_sell_amount: Uint128::new(1) }
    }

    #[test]
    fn valid_config_passes() { assert!(validate_launch_config(&valid_config()).is_ok()); }

    #[test]
    fn invalid_config_is_rejected() {
        let mut config = valid_config();
        config.token_decimals = 19;
        assert!(validate_launch_config(&config).is_err());
    }
}
