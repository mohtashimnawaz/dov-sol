use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct SettleEpoch<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

pub fn handle(ctx: Context<SettleEpoch>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(now >= ctx.accounts.vault.expiry_ts, VaultError::VaultExpired);

    // Placeholder: perform settlement logic
    msg!("Epoch settled. PnL distributed. (placeholder)");
    Ok(())
}
