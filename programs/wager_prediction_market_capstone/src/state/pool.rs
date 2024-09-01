use anchor_lang::prelude::*;

#[account]
pub struct LiquidityPool {
    pub total_liquidity: u64,
    pub owner: Pubkey,
    pub fee_percentage: u8,
    pub is_active: bool,
}

impl LiquidityPool {
    pub const INIT_SPACE: usize = 8 + 
        8 + 
        32 + 
        1 + 
        1; 
}
