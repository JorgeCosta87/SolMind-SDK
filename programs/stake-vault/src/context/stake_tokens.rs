use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        TokenInterface,
        TokenAccount,
        TransferChecked,
        transfer_checked,
    }
};

use crate::state::{StakeConfig, StakingPosition};
use crate::errors::StakeError;

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub deposit_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = deposit_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub deposit_mint_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        has_one = deposit_mint,
        seeds = [b"stake_config", stake_config.admin.key().as_ref()],
        bump = stake_config.bump
    )]
    pub stake_config: Account<'info, StakeConfig>,
    #[account(
        init,
        space = 8 + StakingPosition::INIT_SPACE,
        payer = user,
        seeds = [b"stake_position", stake_config.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub stake_position: Account<'info, StakingPosition>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = deposit_mint,
        associated_token::authority = stake_position,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakeTokens<'info> {
    pub fn stake_tokens(
        &mut self, amount: u64, lock_durations_days: u64, bumps: &StakeTokensBumps
    ) -> Result<()> {
        let clock = Clock::get()?;

        let slots_per_day = 216_000u64;
        let lock_duration_slots = lock_durations_days
            .checked_mul(slots_per_day).ok_or(StakeError::InvalidLockDuration)?; // Improve this

        require!(
            self.stake_config.lock_durations.contains(&lock_durations_days),
            StakeError::InvalidLockDuration,
        );

        let user_to_vault_accounts = TransferChecked {
            from: self.deposit_mint_ata.to_account_info(),
            mint: self.deposit_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(), user_to_vault_accounts
        );

        transfer_checked(
            cpi_ctx, amount, self.deposit_mint.decimals
        )?;

        self.stake_position.set_inner(
            StakingPosition {
                user: self.user.key(),
                stake_config: self.stake_config.key(),
                amount_staked: amount,
                stake_slot: clock.slot,
                unlock_slot: clock.slot
                .checked_add(lock_duration_slots).ok_or(StakeError::InvalidLockDuration)?,
                last_claim_slot: clock.slot,
                total_claimed: 0,
                bump: bumps.stake_position,
            });

        self.stake_config.total_staked += amount;
        self.stake_config.total_positions += 1;

        Ok(())
    }
}