use anchor_lang::prelude::*;

#[constant]
pub const ESCROW_SEED: &[u8] = b"escrow";

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

#[constant]
pub const MAX_FEE_BPS: u16 = 1_000; // 10%
