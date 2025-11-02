use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakingPosition {
    pub user: Pubkey,
    pub stake_config: Pubkey,
    pub amount_staked: u64,
    pub stake_slot: u64,
    pub unlock_slot: u64,
    pub last_claim_slot: u64,
    pub total_claimed: u64,
    pub bump: u8,
}