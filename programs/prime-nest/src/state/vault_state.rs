use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub user: Pubkey,
    pub expiration: i64,
    pub amount: u64,
    pub sol_price_at_initialization: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl VaultState {
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 1 + 1 + 1;
}
