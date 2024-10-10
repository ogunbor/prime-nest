use crate::errors::VaultError;
use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE,
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn initialize_vault(&mut self, lock_duration: i64, bumps: &MakeBumps) -> Result<()> {
        let min_lock_duration: i64 = 2_592_000; // 2,592,000 seconds ( 1 month )

        require!(lock_duration >= min_lock_duration, VaultError::TimeTooShort);

        self.state.set_inner(VaultState {
            user: self.user.key(),
            expiration: Clock::get()?.unix_timestamp + lock_duration,
            vault_bump: bumps.vault,
            state_bump: bumps.state,
        });
        Ok(())
    }
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let min_deposit: u64 = 1_000_000_000; //( 1 sol )

        // Ensure the deposit amount is at least 1 SOL
        if amount < min_deposit {
            return Err(VaultError::DepositTooSmall.into());
        }

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
