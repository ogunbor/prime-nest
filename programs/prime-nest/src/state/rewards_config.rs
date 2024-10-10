use anchor_lang::prelude::*;

#[account]
pub struct RewardsConfig {
    pub rewards_bump: u8,
    pub bump: u8,
}

impl Space for RewardsConfig {
    const INIT_SPACE: usize = 8 + 1 + 1;
}
