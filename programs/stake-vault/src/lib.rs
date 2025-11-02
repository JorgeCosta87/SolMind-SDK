use anchor_lang::prelude::*;

declare_id!("77dFMte18dpEJwG6N4VRr8qqHFYe3hXzbUU8q9KvQNhH");

pub mod state;
pub mod context;

pub use context::*;

#[program]
pub mod stake_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        args:TokenMetadataArgs,
        base_yield_rate: u64,
        lock_durations: Vec<u32>,

    ) -> Result<()> {
        ctx.accounts.initialize(base_yield_rate, lock_durations, &ctx.bumps)?;
        ctx.accounts.create_mint(args)
    }
}