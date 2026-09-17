use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("math overflow")]
    MathOverflow,
    #[error("math underflow")]
    MathUnderflow,
    #[error("division by zero")]
    DivisionByZero,
    #[error("zero amount is not allowed")]
    ZeroAmount,
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("slippage limit was not met")]
    SlippageLimitNotMet,
}
