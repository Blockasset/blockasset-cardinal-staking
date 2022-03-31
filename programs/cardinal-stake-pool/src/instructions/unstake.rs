use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct UnstakeCtx<'info> {
    #[account(mut)]
    stake_entry: Box<Account<'info, StakeEntry>>,

    // stake_entry token accounts
    #[account(mut, constraint =
        stake_entry_original_mint_token_account.amount > 0
        && stake_entry_original_mint_token_account.mint == stake_entry.original_mint
        && stake_entry_original_mint_token_account.owner == stake_entry.key()
        @ ErrorCode::InvalidStakeEntryOriginalMintTokenAccount)]
    stake_entry_original_mint_token_account: Box<Account<'info, TokenAccount>>,

    // user
    #[account(mut, constraint = user.key() == stake_entry.last_staker @ ErrorCode::InvalidUnstakeUser)]
    user: Signer<'info>,
    #[account(mut, constraint =
        user_original_mint_token_account.mint == stake_entry.original_mint
        && user_original_mint_token_account.owner == user.key()
        @ ErrorCode::InvalidUserOriginalMintTokenAccount)]
    user_original_mint_token_account: Box<Account<'info, TokenAccount>>,

    // programs
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<UnstakeCtx>) -> Result<()> {
    let stake_entry = &mut ctx.accounts.stake_entry;

    let stake_entry_seed = &[STAKE_ENTRY_PREFIX.as_bytes(), stake_entry.pool.as_ref(), stake_entry.original_mint.as_ref(), &[stake_entry.bump]];
    let stake_entry_signer = &[&stake_entry_seed[..]];

    // If receipt has been minted, ensure it is back in the stake_entry
    if stake_entry.receipt_mint != None {
        let remaining_accs = &mut ctx.remaining_accounts.iter();
        let stake_entry_receipt_mint_token_account_info = next_account_info(remaining_accs)?;
        let stake_entry_receipt_mint_token_account = Account::<TokenAccount>::try_from(stake_entry_receipt_mint_token_account_info)?;
        if stake_entry_receipt_mint_token_account.mint != stake_entry.receipt_mint.unwrap()
            || stake_entry_receipt_mint_token_account.owner != stake_entry.key()
            || stake_entry_receipt_mint_token_account.amount == 0
        {
            return Err(error!(ErrorCode::InvalidStakeEntryReceiptTokenAccount));
        }
    }

    // give back original mint to user
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.stake_entry_original_mint_token_account.to_account_info(),
        to: ctx.accounts.user_original_mint_token_account.to_account_info(),
        authority: stake_entry.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(stake_entry_signer);
    token::transfer(cpi_context, 1)?;

    // match stake_entry.stake_type {
    //     k if k == StakeType::Locked as u8 => {
    //         let stake_entry_seed = &[STAKE_ENTRY_PREFIX.as_bytes(), stake_entry.pool.as_ref(), stake_entry.original_mint.as_ref(), &[stake_entry.bump]];
    //         let stake_entry_signer = &[&stake_entry_seed[..]];

    //         let cpi_accounts = cardinal_token_manager::cpi::accounts::InvalidateCtx {
    //             token_manager: ctx.accounts.token_manager.to_account_info(),
    //             token_manager_token_account: ctx.accounts.token_manager_token_account.to_account_info(),
    //             mint: ctx.accounts.original_mint.to_account_info(),
    //             invalidator: ctx.accounts.stake_entry.to_account_info(),
    //             collector: ctx.accounts.stake_entry.to_account_info(),
    //             recipient_token_account: ctx.accounts.user_original_mint_token_account.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //         };
    //         let cpi_ctx = CpiContext::new(ctx.accounts.token_manager_program.to_account_info(), cpi_accounts)
    //             .with_signer(stake_entry_signer)
    //             .with_remaining_accounts(remaining_accs);
    //         cardinal_token_manager::cpi::invalidate(cpi_ctx)?;
    //     }
    //     k if k == StakeType::Escrow as u8 => {
    //         // give back original mint to user
    //         let cpi_accounts = token::Transfer {
    //             from: ctx.accounts.stake_entry_original_mint_token_account.to_account_info(),
    //             to: ctx.accounts.user_original_mint_token_account.to_account_info(),
    //             authority: stake_entry.to_account_info(),
    //         };
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(stake_entry_signer);
    //         token::transfer(cpi_context, 1)?;
    //     }
    //     _ => return Err(error!(ErrorCode::InvalidStakeType)),
    // }
    // saturing add will stop at max u64

    stake_entry.total_stake_seconds = stake_entry
        .total_stake_seconds
        .saturating_add(Clock::get().unwrap().unix_timestamp.checked_sub(stake_entry.last_staked_at).unwrap());
    stake_entry.last_staker = Pubkey::default();
    stake_entry.receipt_mint_claimed = false;
    stake_entry.stake_type = StakeType::Unstaked as u8;

    Ok(())
}
