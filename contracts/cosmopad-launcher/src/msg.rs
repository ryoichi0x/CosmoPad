use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub atom_denom: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimals: u8,
    pub fee_bps: u16,
    pub min_buy_amount: Uint128,
    pub min_sell_amount: Uint128,
    pub virtual_atom_reserve: Uint128,
    pub virtual_token_reserve: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    Placeholder,
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    CurveState {},
    InitialPrice {},
    BuyEstimate {
        atom_in: Uint128,
        min_tokens_out: Uint128,
    },
    SellEstimate {
        token_in: Uint128,
        min_atom_out: Uint128,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub atom_denom: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimals: u8,
    pub fee_bps: u16,
    pub min_buy_amount: Uint128,
    pub min_sell_amount: Uint128,
}

#[cw_serde]
pub struct CurveStateResponse {
    pub atom_reserve: Uint128,
    pub token_reserve: Uint128,
    pub k: String,
}

#[cw_serde]
pub struct PriceResponse {
    pub value: Uint128,
}

#[cw_serde]
pub struct EstimateResponse {
    pub input: Uint128,
    pub output: Uint128,
    pub minimum_output: Uint128,
}
