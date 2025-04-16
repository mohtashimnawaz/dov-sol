use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    
    pub name: String,
    pub authority: Pubkey,
    pub strike_price: u64,
    pub expiry_ts: i64,
    pub total_deposits: u64,
    pub bump: u8,
    pub option_token_mint: Pubkey, // NEW: Option token mint
    pub user_option_tokens: u64,    // NEW: Tracks total minted option tokens
}

impl Vault {
    pub const LEN: usize = 8 + 4 + 32 + 8 + 8 + 8 + 1 + 32 + 8;
}

// errors.rs
