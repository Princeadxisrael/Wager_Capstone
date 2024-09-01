
use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub wallet: Pubkey,
    pub total_bets: u64,
    pub total_winnings: u64,
    pub total_losses: u64,
    pub active_bets: Vec<Pubkey>,
}

impl User {
    pub const INIT_SPACE: usize = 8 + 
        32 + 
        8 + 
        8 + 
        8 + 
        4 + (32 * 10); // active_bets (assuming max 10 active bets, each 32 bytes for Pubkey)
}
