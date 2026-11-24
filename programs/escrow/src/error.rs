use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Fee exceeds the maximum allowed")]
    FeeTooHigh,
}
