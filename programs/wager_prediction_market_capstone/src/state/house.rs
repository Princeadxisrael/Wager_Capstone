use anchor_lang::prelude::*;

#[account]
pub struct HousePool {
    pub total_funds: u64,
    pub owner: Pubkey,
    pub fee_percentage: u8,
    pub is_active: bool,
    pub total_bets_handled: u64,
    pub total_payouts: u64,
}

impl HousePool {
    pub const INIT_SPACE: usize = 8 + 
        8 + 
        32 + 
        1 + 
        1 + 
        8 + 
        8; 
}
