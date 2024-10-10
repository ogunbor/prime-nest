use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::RewardsConfig;

#[derive(Accounts)]
pub struct InitializeRewardsConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init, 
        payer = admin,
        seeds = [b"config".as_ref()],
        bump,
        space = RewardsConfig::INIT_SPACE,
    )]
    pub config: Account<'info, RewardsConfig>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeRewardsConfig<'info> {
    pub fn initialize_rewards_config(&mut self, bumps: &InitializeRewardsConfigBumps) -> Result<()> {
        self.config.set_inner(RewardsConfig {
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });

        Ok(())
    }
}