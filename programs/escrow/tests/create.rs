use anchor_lang::system_program;
use anchor_litesvm::{
    AnchorContext, AnchorLiteSVM, AssertionHelpers, Keypair, Pubkey, Signer, TestHelpers,
};
use escrow::{Escrow, EscrowState, ESCROW_SEED, MAX_FEE_BPS, VAULT_SEED};
use litesvm_utils::TransactionResult;

// Setup an account
struct Setup {
    ctx: AnchorContext,
    creator: Keypair,
    mint: Pubkey,
}

fn setup() -> Setup {
    // create a fake network and "deploy" our program at its address
    let mut ctx = AnchorLiteSVM::build_with_program(
        escrow::ID,
        include_bytes!("../../../target/deploy/escrow.so"),
    );
    // make a new wallet with 10 SOL -> lamport vars
    let creator = ctx.svm.create_funded_account(10_000_000_000).unwrap();

    // make a fake token (like USDC, 6 decimals) with the creator as its mint authority
    let mint = ctx.svm.create_token_mint(&creator, 6).unwrap().pubkey();
    Setup { ctx, creator, mint }
}

// Terms of the Test Escrow (One Escrow)
struct Terms {
    escrow_id: u64,
    referee: Pubkey,
    fee_recipient: Pubkey,
    fee_bps: u16,
    deadline: i64,
}

impl Terms {
    fn valid() -> Self {
        Terms {
            escrow_id: 1,
            referee: Keypair::new().pubkey(),
            fee_recipient: Keypair::new().pubkey(),
            fee_bps: 250,
            deadline: 1_900_000_000,
        }
    }
}

// create the escrow address
fn escrow_address(s: &Setup, escrow_id: u64) -> Pubkey {
    // compute the address, using ESCROW_SEED constant as the program
    s.ctx.svm.get_pda(
        &[
            ESCROW_SEED,
            s.creator.pubkey().as_ref(),
            &escrow_id.to_le_bytes(),
        ],
        &escrow::ID,
    )
}

// create the vault address
fn vault_address(s: &Setup, escrow: &Pubkey) -> Pubkey {
    // compute the address, using the VAULT_SEED constant as the program
    s.ctx
        .svm
        .get_pda(&[VAULT_SEED, escrow.as_ref()], &escrow::ID)
}

// create the escrow
fn create_escrow(s: &mut Setup, t: &Terms) -> TransactionResult {
    let escrow = escrow_address(s, t.escrow_id);
    let vault = vault_address(s, &escrow);

    // the instruction
    let ix = s
        .ctx
        .program()
        .accounts(escrow::accounts::Create {
            creator: s.creator.pubkey(),
            mint: s.mint,
            escrow,
            vault,
            system_program: system_program::ID,
            token_program: anchor_spl::token::ID,
        })
        .args(escrow::instruction::Create {
            escrow_id: t.escrow_id,
            referee: t.referee,
            fee_recipient: t.fee_recipient,
            fee_bps: t.fee_bps,
            deadline: t.deadline,
        })
        .instruction()
        .unwrap();

    // send the instruction as a transaction signed by the creator.
    // returns a result
    s.ctx.execute_instruction(ix, &[&s.creator]).unwrap()
}

#[test]
fn create_stores_the_terms() {
    let mut s = setup();
    let terms = Terms::valid();

    create_escrow(&mut s, &terms).assert_success();

    let escrow_pda = escrow_address(&s, terms.escrow_id);
    let escrow: Escrow = s.ctx.get_account(&escrow_pda).unwrap();
    assert_eq!(escrow.creator, s.creator.pubkey());
    assert_eq!(escrow.mint, s.mint);
    assert_eq!(escrow.referee, terms.referee);
    assert_eq!(escrow.fee_recipient, terms.fee_recipient);
    assert_eq!(escrow.fee_bps, terms.fee_bps);
    assert_eq!(escrow.deadline, terms.deadline);
    assert_eq!(escrow.total_deposited, 0);
    assert!(escrow.state == EscrowState::Open);

    let vault = vault_address(&s, &escrow_pda);
    s.ctx.svm.assert_token_balance(&vault, 0);
}

#[test]
fn create_rejects_fee_above_cap() {
    let mut s = setup();
    let terms = Terms {
        fee_bps: MAX_FEE_BPS + 1,
        ..Terms::valid()
    };

    create_escrow(&mut s, &terms).assert_anchor_error("FeeTooHigh");
}

// opt: test if fee is exactly at the cap
#[test]
fn create_accepts_fee_exactly_at_cap() {
    let mut s = setup();
    let terms = Terms {
        fee_bps: MAX_FEE_BPS,
        ..Terms::valid()
    };

    create_escrow(&mut s, &terms).assert_success();
}
