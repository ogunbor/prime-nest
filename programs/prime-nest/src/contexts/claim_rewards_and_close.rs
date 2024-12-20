use crate::errors::VaultError;
use crate::state::{RewardsConfig, VaultState};
use crate::utils::get_feed_id_from_hex;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::{MAXIMUM_AGE, SOL_ID};

#[derive(Accounts)]
pub struct ClaimAndClose<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub user_reward_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, RewardsConfig>,
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user,
        has_one = user,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimAndClose<'info> {
    pub fn claim_rewards_and_close(&mut self) -> Result<()> {
        // Get the current on-chain timestamp
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        // Ensure the vault has reached its expiration time
        require!(
            current_timestamp >= self.vault_state.expiration,
            VaultError::VaultNotYetExpired
        );

        // Parse feed ID from hex
        let feed_id: [u8; 32] = get_feed_id_from_hex(SOL_ID)?;

        // Get the price update
        let price_data =
            self.price_update
                .get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

        // Access the price
        let price: i64 = price_data.price;

        // Ensure price is non-negative before converting to u64
        let sol_price_at_claim: u64 = price
            .try_into()
            .map_err(|_| VaultError::InvalidPriceConversion)?; // Handle possible conversion errors

        let accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[b"config", &[self.config.bump]]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        let amount_to_mint = if self.vault_state.sol_price_at_initialization < sol_price_at_claim {
            self.vault_state
                .amount
                .checked_mul(self.vault_state.expiration as u64)
                .and_then(|res| res.checked_mul(10_u64.pow(self.rewards_mint.decimals as u32)))
                .and_then(|res| res.checked_add(10_000))
                .ok_or(VaultError::ArithmeticOverflow)?
        } else {
            self.vault_state
                .amount
                .checked_mul(self.vault_state.expiration as u64)
                .and_then(|res| res.checked_mul(10_u64.pow(self.rewards_mint.decimals as u32)))
                .ok_or(VaultError::ArithmeticOverflow)?
        };

        // Call the mint_to function with the calculated amount
        mint_to(ctx, amount_to_mint)?;

        // 2: close the vault account and get lamports
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer all SOL in the vault to the user
        transfer(cpi_ctx, self.vault_state.amount)?;

        Ok(())
    }
}
