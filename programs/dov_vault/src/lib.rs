use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use crate::errors::VaultError;

// Import instruction handlers
use crate::instructions::initialize_vault::handle as initialize_vault_handle;
use crate::instructions::deposit::handle as deposit_handle;
use crate::instructions::withdraw::handle as withdraw_handle;
use crate::instructions::sell_options::handle as sell_options_handle;
use crate::instructions::settle_epoch::handle as settle_epoch_handle;


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
