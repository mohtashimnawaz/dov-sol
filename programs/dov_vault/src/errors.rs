use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Vault has expired")]
    VaultExpired,
    #[msg("Insufficient funds to withdraw")]
    InsufficientFunds,
    #[msg("Only the vault authority can perform this action")]
    Unauthorized,
}