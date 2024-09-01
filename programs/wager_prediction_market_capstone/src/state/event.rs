use anchor_lang::prelude::*;

#[account]
pub struct Event {
    pub id: u64,
    pub outcomes: Vec<String>,
    pub odds: Vec<u64>,
    pub is_active: bool,
    pub total_bets: Vec<u64>,
    pub winning_outcome: Option<usize>,
}

impl Event {
    pub const INIT_SPACE: usize = 8 + 
        8 + 
        4 + (32 * 3) + // outcomes (assuming max 3 outcomes, each 32 bytes)
        4 + (8 * 3) + // odds (assuming max 3 odds, each 8 bytes)
        1 + 
        4 + (8 * 3) + // total_bets (assuming max 3 total_bets, each 8 bytes)
        1 + 8; // winning_outcome (Option<usize>)
}
