use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("The vault has not yet expired")]
    VaultNotYetExpired,
    #[msg("Invalid withdrawal amount")]
    InvalidWithdrawalAmount,
    #[msg("You are not authorized to do this")]
    Unauthorized,
    #[msg("Minimum 1 month for lockup period")]
    TimeTooShort,
    #[msg("Minimum 1 sol for deposit")]
    DepositTooSmall,
    #[msg("The vault is expired already")]
    VaultExpired,
    #[msg("You dont have access to rewards")]
    NoRewardsAccess,
    #[msg("Failed to CPI")]
    CPIFailed,
}
