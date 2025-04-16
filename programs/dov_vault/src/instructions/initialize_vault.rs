// instructions/initialize_vault.rs
use anchor_lang::prelude::*;
use crate::state::*;
use crate::state::vault::Vault;

#[derive(Accounts)]
#[instruction(vault_name: String)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"vault", vault_name.as_bytes()],
        bump,
        payer = authority,
        space = 8 + Vault::LEN
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<InitializeVault>,
    vault_name: String,
    strike_price: u64,
    expiry_ts: i64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.name = vault_name;
    vault.authority = ctx.accounts.authority.key();
    vault.strike_price = strike_price;
    vault.expiry_ts = expiry_ts;
    vault.total_deposits = 0;
    vault.bump = *ctx.bumps.get("vault").unwrap();
    Ok(())
}
