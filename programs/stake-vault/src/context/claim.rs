use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        MintTo,
        mint_to,
        TokenInterface,
        TokenAccount,
        TransferChecked,
        transfer_checked,
    }
};

use crate::state::{StakeConfig, StakingPosition};
use crate::errors::StakeError;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub reward_mint_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        has_one = reward_mint,
        seeds = [b"stake_config", stake_config.admin.key().as_ref()],
        bump = stake_config.bump
    )]
    pub stake_config: Account<'info, StakeConfig>,
    #[account(
        mut,
        has_one = user,
        seeds = [b"stake_position", stake_config.key().as_ref(), user.key().as_ref()],
        bump = stake_config.bump,
    )]
    pub stake_position: Account<'info, StakingPosition>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn stake_tokens(
        &mut self,
    ) -> Result<()> {
        let clock = Clock::get()?;

        // Calculate slots since last claim
        let slots_since_claim = clock.slot
            .saturating_sub(self.stake_position.last_claim_slot);
        
        require!(slots_since_claim > 0, StakeError::NoYieldAccrued);
       
        let slots_per_day = 216_000u64;
        let slots_per_year = 365u64
            .checked_mul(slots_per_day)
            .ok_or(StakeError::SomentingWentWrong)?; // 78,840,000 slots
        
        require!(slots_per_year > 0, StakeError::SomentingWentWrong);
 
        let yield_amount = (self.stake_position.amount_staked as u128)
            .checked_mul(self.stake_position.yield_rate as u128)
            .ok_or(StakeError::SomentingWentWrong)? 
            .checked_mul(slots_since_claim as u128)
            .ok_or(StakeError::SomentingWentWrong)? 
            .checked_div(10000u128) 
            .ok_or(StakeError::SomentingWentWrong)? 
            .checked_div(slots_per_year as u128)
            .ok_or(StakeError::SomentingWentWrong)? as u64; 

        require!(yield_amount > 0, StakeError::NoYieldAccrued);

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"stake_config",
            self.stake_config.to_account_info().key.as_ref(),
            self.stake_config.admin.as_ref(),
            &[self.stake_config.bump]
        ]];

        let reward_mint_accounts = MintTo  {
            mint: self.reward_mint.to_account_info(),
            to: self.user.to_account_info(),
            authority: self.stake_position.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), reward_mint_accounts, &signer_seeds
        );

        mint_to(cpi_ctx, yield_amount)?;

        Ok(())
    }
}