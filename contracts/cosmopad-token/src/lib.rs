//! CosmoPad's standard CW20 token contract.
//!
//! We intentionally reuse cw20-base instead of rewriting token accounting.
//! This reduces risk and gives us the standard CW20 interface used by Cosmos tools.

pub use cw20_base::contract::{execute, instantiate, query};
pub use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{from_json, testing::{mock_dependencies, mock_env, mock_info}, Addr, Uint128};
    use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};

    #[test]
    fn creates_token_with_initial_balance() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name: "CosmoPad Token".to_string(),
            symbol: "CPT".to_string(),
            decimals: 6,
            initial_balances: vec![cw20::Cw20Coin {
                address: "alice".to_string(),
                amount: Uint128::new(1_000_000),
            }],
            mint: None,
            marketing: None,
        };

        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();

        let response: TokenInfoResponse = from_json(
            query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::TokenInfo {},
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(response.name, "CosmoPad Token");
        assert_eq!(response.symbol, "CPT");
        assert_eq!(response.decimals, 6);
        assert_eq!(response.total_supply, Uint128::new(1_000_000));

        let balance: BalanceResponse = from_json(
            query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::Balance { address: "alice".to_string() },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(balance.balance, Uint128::new(1_000_000));
    }

    #[test]
    fn transfers_token_between_accounts() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name: "CosmoPad Token".to_string(),
            symbol: "CPT".to_string(),
            decimals: 6,
            initial_balances: vec![cw20::Cw20Coin {
                address: "alice".to_string(),
                amount: Uint128::new(100),
            }],
            mint: None,
            marketing: None,
        };
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &[]),
            ExecuteMsg::Transfer { recipient: "bob".to_string(), amount: Uint128::new(40) },
        )
        .unwrap();

        let balance: BalanceResponse = from_json(
            query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::Balance { address: "bob".to_string() },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(balance.balance, Uint128::new(40));
    }

    #[test]
    fn invalid_address_is_rejected_by_token_contract() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name: "CosmoPad Token".to_string(),
            symbol: "CPT".to_string(),
            decimals: 6,
            initial_balances: vec![cw20::Cw20Coin {
                address: "not an address".to_string(),
                amount: Uint128::new(1),
            }],
            mint: None,
            marketing: None,
        };
        assert!(instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).is_err());
    }
}
