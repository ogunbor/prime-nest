use crate::errors::VaultError;
use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use hex;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

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
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn initialize_vault(&mut self, lock_duration: i64, bumps: &MakeBumps) -> Result<()> {
        // Time in seconds for locking vault
        let min_lock_duration: i64 = 2_592_000;

        // Time in seconds for price feed from pyth
        let maximum_age: u64 = 30;

        // Parse feed ID from hex
        let feed_id: [u8; 32] = get_feed_id_from_hex(
            "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
        )?;

        // Get the price update
        let price_data =
            self.price_update
                .get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

        // Access the price
        let price: i64 = price_data.price; // price is i64

        // Ensure price is non-negative before converting to u64
        let sol_price_at_initialization: u64 = price
            .try_into()
            .map_err(|_| VaultError::InvalidPriceConversion)?; // Handle possible conversion errors

        // Check if the lock duration is sufficient
        require!(lock_duration >= min_lock_duration, VaultError::TimeTooShort);

        // Set up initial state values
        self.state.set_inner(VaultState {
            user: self.user.key(),
            expiration: Clock::get()?.unix_timestamp + lock_duration,
            amount: 0,
            sol_price_at_initialization,
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

        self.state.amount += amount;
        assert_eq!(self.state.amount, amount);
        Ok(())
    }
}

// Helper function for parsing hex feed ID
fn get_feed_id_from_hex(hex_str: &str) -> Result<[u8; 32]> {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(hex_str, &mut bytes).map_err(|_| error!(VaultError::InvalidHex))?;
    Ok(bytes)
}
