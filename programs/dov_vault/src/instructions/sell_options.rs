use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct SellOptions<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

pub fn handle(ctx: Context<SellOptions>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(now < ctx.accounts.vault.expiry_ts, VaultError::VaultExpired);

    // TODO: Integrate with Zeta/PsyOptions to sell options
    msg!("Options sold successfully (placeholder)");
    Ok(())
}