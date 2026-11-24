use anchor_lang::prelude::*;

use anchor_lang::prelude::*;

pub const MAX_FEE_BPS: u16 = 1_000; // 10% hard cap

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub creator: Pubkey,
    pub referee: Pubkey, // only this key can settle
    pub mint: Pubkey,    // which token is held
    pub fee_recipient: Pubkey,
    pub fee_bps: u16,
    pub deadline: i64, // unix time; after this, refunds open up
    pub total_deposited: u64,
    pub state: EscrowState,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum EscrowState {
    Open,
    Locked,
    Settled,
    Refundable,
}
