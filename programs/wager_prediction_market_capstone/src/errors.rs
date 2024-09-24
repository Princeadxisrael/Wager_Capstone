use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Event is already resolved")]
    EventAlreadyResolved,
    #[msg("Invalid winning outcome")]
    InvalidWinningOutcome,
    #[msg("Insufficient balance in the vault")]
    InsufficientVaultBalance,
    #[msg("Payout calculation overflow")]
    PayoutOverflow,
    #[msg("Payout mismatch")]
    PayoutMismatch,
    #[msg("Cannot Enter")]
    CannotPlacebet,
    #[msg("Cannot Enter")]
    CannotClaim,
    #[msg("Cannot Close")]
    CannotClose,
    #[msg("amount is too big to parse to u32")]
    AmountTooBig,
    #[msg("The event is currently inactive.")]
    EventInactive,
    #[msg("Invalid outcome index provided.")]
    InvalidOutcomeIndex,
    #[msg("Bet has not been resolved yet.")]
    BetNotResolved,
    #[msg("Unauthorized event resolution attempt")]
    UnauthorizedResolution,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Insufficient Liquidity")]
    InsufficientLiquidity,
    
}