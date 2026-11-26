use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub escrow_id: u64,
    pub creator: Pubkey,
    pub referee: Pubkey,
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub fee_bps: u16,
    pub deadline: i64,
    pub total_deposited: u64,
    pub allocated: u64, // how much the referee has promised out so far
    pub deposit_count: u32,
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
