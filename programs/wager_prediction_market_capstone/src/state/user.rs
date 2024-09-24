
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub wallet: Pubkey,
    pub total_bets: u64,
    pub total_winnings: u64,
    pub total_losses: u64,
    #[max_len(10,32)]
    pub active_bets: Vec<Pubkey>,
    pub bump: u8,
}


