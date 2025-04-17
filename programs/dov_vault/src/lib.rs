use anchor_lang::prelude::*;

// Import all instructions at once
mod state;
mod instructions;
mod errors;

// Direct imports for specific instruction handlers
use instructions::initialize_vault::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use instructions::sell_options::*;
use instructions::settle_epoch::*;

use crate::errors::VaultError;

declare_id!("CyAmdmRwrJzbPS84TkFxdH9KWfLMMSgGsXAaYZpTmmXs");

#[program]
pub mod dov_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_name: String,
        strike_price: u64,
        expiry_ts: i64,
    ) -> Result<()> {
        initialize_vault_handle(ctx, vault_name, strike_price, expiry_ts)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit_handle(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw_handle(ctx, amount)
    }

    pub fn sell_options(ctx: Context<SellOptions>) -> Result<()> {
        sell_options_handle(ctx)
    }

    pub fn settle_epoch(ctx: Context<SettleEpoch>) -> Result<()> {
        settle_epoch_handle(ctx)
    }
}
