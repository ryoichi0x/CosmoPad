use cosmwasm_std::{Uint128, Uint256};

use crate::error::ContractError;

fn non_zero(value: Uint128, name: &str) -> Result<(), ContractError> {
    if value.is_zero() { return Err(ContractError::InvalidInput(format!("{name} must be greater than zero"))); }
    Ok(())
}

/// Integer-only initial price. It is a floored quote in ATOM units per token unit.
pub fn initial_price(atom_reserve: Uint128, token_reserve: Uint128) -> Result<Uint128, ContractError> {
    non_zero(atom_reserve, "atom_reserve")?;
    non_zero(token_reserve, "token_reserve")?;
    atom_reserve.checked_div(token_reserve).map_err(|_| ContractError::DivisionByZero)
}

/// Returns token output for `atom_in`, without transferring anything.
pub fn calculate_buy(atom_in: Uint128, atom_reserve: Uint128, token_reserve: Uint128) -> Result<Uint128, ContractError> {
    non_zero(atom_in, "atom_in")?;
    non_zero(atom_reserve, "atom_reserve")?;
    non_zero(token_reserve, "token_reserve")?;

    let new_atom_reserve = atom_reserve.checked_add(atom_in).map_err(|_| ContractError::MathOverflow)?;
    let k = Uint256::from(atom_reserve).checked_mul(Uint256::from(token_reserve)).map_err(|_| ContractError::MathOverflow)?;
    let new_token = k.checked_div(Uint256::from(new_atom_reserve)).map_err(|_| ContractError::DivisionByZero)?;
    let new_token = Uint128::try_from(new_token).map_err(|_| ContractError::MathOverflow)?;
    token_reserve.checked_sub(new_token).map_err(|_| ContractError::MathUnderflow)
}

/// Returns ATOM output for `token_in`, without transferring anything.
pub fn calculate_sell(token_in: Uint128, atom_reserve: Uint128, token_reserve: Uint128) -> Result<Uint128, ContractError> {
    non_zero(token_in, "token_in")?;
    non_zero(atom_reserve, "atom_reserve")?;
    non_zero(token_reserve, "token_reserve")?;

    let new_token_reserve = token_reserve.checked_add(token_in).map_err(|_| ContractError::MathOverflow)?;
    let k = Uint256::from(atom_reserve).checked_mul(Uint256::from(token_reserve)).map_err(|_| ContractError::MathOverflow)?;
    let new_atom = k.checked_div(Uint256::from(new_token_reserve)).map_err(|_| ContractError::DivisionByZero)?;
    let new_atom = Uint128::try_from(new_atom).map_err(|_| ContractError::MathOverflow)?;
    atom_reserve.checked_sub(new_atom).map_err(|_| ContractError::MathUnderflow)
}

pub fn enforce_minimum(actual: Uint128, minimum: Uint128) -> Result<(), ContractError> {
    if actual < minimum { return Err(ContractError::SlippageLimitNotMet); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const X: Uint128 = Uint128::new(1_000);
    const Y: Uint128 = Uint128::new(1_000);

    #[test]
    fn initial_price_is_one_for_equal_reserves() { assert_eq!(initial_price(X, Y).unwrap(), Uint128::new(1)); }

    #[test]
    fn buy_calculation_is_91() { assert_eq!(calculate_buy(Uint128::new(100), X, Y).unwrap(), Uint128::new(91)); }

    #[test]
    fn sell_calculation_is_91() { assert_eq!(calculate_sell(Uint128::new(100), X, Y).unwrap(), Uint128::new(91)); }

    #[test]
    fn zero_input_is_rejected() {
        assert_eq!(calculate_buy(Uint128::zero(), X, Y), Err(ContractError::InvalidInput("atom_in must be greater than zero".into())));
        assert_eq!(calculate_sell(Uint128::zero(), X, Y), Err(ContractError::InvalidInput("token_in must be greater than zero".into())));
    }

    #[test]
    fn invalid_and_overflow_values_are_rejected() {
        assert!(initial_price(Uint128::zero(), Y).is_err());
        assert!(calculate_buy(Uint128::new(1), Uint128::MAX, Y).is_err());
        assert!(calculate_sell(Uint128::new(1), X, Uint128::MAX).is_err());
    }

    #[test]
    fn buy_then_sell_uses_updated_reserves() {
        let bought = calculate_buy(Uint128::new(100), X, Y).unwrap();
        let returned = calculate_sell(bought, Uint128::new(1_100), Uint128::new(909)).unwrap();
        assert_eq!(bought, Uint128::new(91));
        // Integer floor rounding means this is 101 rather than exactly 100.
        assert_eq!(returned, Uint128::new(101));
    }

    #[test]
    fn slippage_limit_is_checked() {
        assert!(enforce_minimum(Uint128::new(90), Uint128::new(91)).is_err());
        assert!(enforce_minimum(Uint128::new(91), Uint128::new(91)).is_ok());
    }
}
