use anchor_lang::error_code;

#[error_code]
pub enum BetError {
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
}