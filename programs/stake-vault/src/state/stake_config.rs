use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub admin: Pubkey,
    pub deposit_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub base_yield_rate: u64, 
    #[max_len(5, 5)]
    pub lock_durations: Vec<u64>, // Lock days
    pub total_staked: u64,
    pub total_positions: u64,
    pub bump: u8,
}