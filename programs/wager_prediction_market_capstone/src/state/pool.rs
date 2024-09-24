use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LiquidityPool {
    pub total_liquidity: u64,
    pub locked_liquidity: u64,
    pub live_liquidity: u64,
    pub pending_bets: u64, 
    pub owner: Pubkey,
    pub fee_percentage: u8,
    pub is_active: bool,
    pub house_pool_usdt: Pubkey,
    pub betting_pool_usdt: Pubkey,
    pub insurance_fund_usdt: Pubkey,
    pub wager_foundation_proceeds_usdt: Pubkey,
    #[max_len(10)]
    pub events: Vec<Pubkey>,
    pub bump: u8
}

impl LiquidityPool{
    pub fn add_event(&mut self, event: Pubkey) -> Result<()> {
        // Check if the events vector is already at maximum capacity
        if self.events.len() >= 10 {
            return Err(ErrorCode::MaxEventsReached.into());
        }

        // Add the new event
        self.events.push(event);
        Ok(())
    }

    pub fn remove_event(&mut self, event: &Pubkey) -> Result<()> {
        if let Some(index) = self.events.iter().position(|&e| e == *event) {
            self.events.remove(index);
            Ok(())
        } else {
            Err(ErrorCode::EventNotFound.into())
        }
    }

    pub fn has_event(&self, event: &Pubkey) -> bool {
        self.events.contains(event)
    }
}

#[error_code]
pub enum ErrorCode {
    MaxEventsReached,
    EventNotFound,
}
