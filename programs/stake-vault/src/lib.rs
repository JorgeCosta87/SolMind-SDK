use anchor_lang::prelude::*;

declare_id!("77dFMte18dpEJwG6N4VRr8qqHFYe3hXzbUU8q9KvQNhH");

pub mod state;
pub mod context;
pub mod errors;

pub use context::*;

#[program]
pub mod stake_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        args:TokenMetadataArgs,
        base_yield_rate: u64,
        lock_durations: Vec<u64>,

    ) -> Result<()> {
        ctx.accounts.initialize(base_yield_rate, lock_durations, &ctx.bumps)?;
        ctx.accounts.create_mint(args)
    }

    pub fn stake_tokens(
        ctx: Context<StakeTokens>,
        amount: u64,
        lock_durations_days: u64
    ) -> Result<()>{
        ctx.accounts.stake_tokens(amount, lock_durations_days, &ctx.bumps)
    }
}