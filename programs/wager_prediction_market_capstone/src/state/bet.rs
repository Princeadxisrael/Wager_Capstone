use anchor_lang::prelude::*;

use crate::state::Event;

#[account]
pub struct Bet {
    pub bettor: Pubkey,
    pub event: Account<'_, Event>,
    pub outcome_index: u8,
    pub amount: u64,
    pub odds_at_placement: u64,
    pub timestamp: i64,
    pub is_resolved: bool,
    pub payout: u64,
}

