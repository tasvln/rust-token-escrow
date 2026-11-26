pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("J1kfcoj8b3wLLsmYac6QAcQ8kFynWFqTyrf1MKS8uvXj");

#[program]
pub mod escrow {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        escrow_id: u64,
        referee: Pubkey,
        fee_recipient: Pubkey,
        fee_bps: u16,
        deadline: i64,
    ) -> Result<()> {
        instructions::create::handle_create(
            ctx,
            escrow_id,
            referee,
            fee_recipient,
            fee_bps,
            deadline,
        )
    }
}
