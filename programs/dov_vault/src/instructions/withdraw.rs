use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::vault::Vault;
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault.name.as_bytes()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(ctx.accounts.vault.total_deposits >= amount, VaultError::InsufficientFunds);

    let seeds = &[
        b"vault",
        ctx.accounts.vault.name.as_bytes(),
        &[ctx.accounts.vault.bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, amount)?;
    ctx.accounts.vault.total_deposits -= amount;
    Ok(())
}