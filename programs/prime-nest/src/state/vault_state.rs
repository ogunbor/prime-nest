use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub user: Pubkey,
    pub expiration: i64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl VaultState {
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 1 + 1;
}
