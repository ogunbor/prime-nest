use anchor_lang::prelude::*;

declare_id!("6iXEfCzmVMN8C5CgRSXpYPcUeRpLJu2o8R5M5uBpjDyt");

pub mod state;
pub use state::*;
pub mod contexts;
pub mod errors;
pub use contexts::*;

#[program]
pub mod prime_nest {
    use super::*;

    pub fn initialize_vault(ctx: Context<Make>, lock_duration: i64, amount: u64) -> Result<()> {
        ctx.accounts.initialize_vault(lock_duration, &ctx.bumps)?;
        ctx.accounts.deposit(amount)
    }

    pub fn initialize_rewards_config(ctx: Context<InitializeRewardsConfig>) -> Result<()> {
        ctx.accounts.initialize_rewards_config(&ctx.bumps)
    }

    pub fn premature_close(ctx: Context<PrematureClose>) -> Result<()> {
        ctx.accounts.premature_close()?;
        Ok(())
    }
}
