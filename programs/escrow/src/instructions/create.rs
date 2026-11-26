use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::{ESCROW_SEED, VAULT_SEED};
use crate::state::{Escrow, EscrowState};

// create struct -> data created or params created
#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct Create<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [ESCROW_SEED, creator.key().as_ref(), &escrow_id.to_le_bytes()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = escrow,
        token::token_program = token_program,
        seeds = [VAULT_SEED, escrow.key().as_ref()],
        bump,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

// main create function
pub fn handle_create(
    ctx: Context<Create>,
    escrow_id: u64,
    referee: Pubkey,
    fee_recipient: Pubkey,
    fee_bps: u16,
    deadline: i64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    escrow.escrow_id = escrow_id;
    escrow.creator = ctx.accounts.creator.key();
    escrow.referee = referee;
    escrow.mint = ctx.accounts.mint.key();
    escrow.fee_recipient = fee_recipient;
    escrow.fee_bps = fee_bps;
    escrow.deadline = deadline;
    escrow.total_deposited = 0;
    escrow.allocated = 0;
    escrow.deposit_count = 0;
    escrow.state = EscrowState::Open;
    escrow.bump = ctx.bumps.escrow;

    Ok(())
}
