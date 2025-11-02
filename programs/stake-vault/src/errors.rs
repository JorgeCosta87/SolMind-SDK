use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Not enough funds.")]
    InsufficientFunds,
    #[msg("Invalid lock duration")]
    InvalidLockDuration,
}