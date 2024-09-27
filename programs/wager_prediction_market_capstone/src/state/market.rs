use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::state::event::EventType;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub event_type: EventType,
    #[max_len(50)]
    pub description: String,
    #[max_len(6,50)]
    pub possible_outcomes: Vec<String>,
    #[max_len(6, 8)]
    pub outcome_liquidity: Vec<u64>,
    pub total_liquidity: u64,
    pub oracle_feed: Pubkey,
    pub result: MarketOutcome, // Index of the winning outcome
    pub start_time: i64,
    pub end_time: i64,
    pub is_settled: bool,
    pub house_pool: Pubkey,
    pub market_authority: Pubkey, 
    pub creation_time: i64, 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MarketParams {
    pub event_id: u64,
    pub event_type: EventType,
    pub description: String,
    pub possible_outcomes: Vec<String>,
    pub odds: Vec<u64>,
    pub start_time: i64,
    pub end_time: i64,
    pub oracle_feed: Pubkey,
    pub house_pool: Pubkey,
}

#[derive(InitSpace,AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MarketOutcome {
    MarketSide0Won,
    MarketSide1Won,
    MarketSide2Won,
    NotYetCommenced,
    Commenced,
    Settled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MarketWinner {
    Side0,
    Side1,
    Side2,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MarketState {
    NotYetCommenced,
    Commenced,
    Settled(MarketWinner),
}

impl From<MarketOutcome> for &'static str {
    fn from(val: MarketOutcome) -> Self {
        match val {
            MarketOutcome::MarketSide0Won => "Market side 0 won",
            MarketOutcome::MarketSide1Won => "Market side 1 won",
            MarketOutcome::MarketSide2Won => "Market side 2 won",
            MarketOutcome::NotYetCommenced => "Not yet commenced",
            MarketOutcome::Commenced => "Commenced",
            MarketOutcome::Settled => "Settled",
        }
    }
}

impl Market {
    pub fn set_data(
        &mut self,
        market_params: MarketParams,
        market_authority: Pubkey,
        house_pool: Pubkey,
    ) -> Result<()> {
        if market_params.possible_outcomes.len() > 3 || market_params.possible_outcomes.is_empty() {
            return Err(ErrorCode::InvalidOutcomesCount.into());
        }
        
        self.event_type = market_params.event_type;
        self.description = market_params.description;
        self.possible_outcomes = market_params.possible_outcomes;
        self.outcome_liquidity = vec![0; self.possible_outcomes.len()];
        self.total_liquidity = 0;
        self.oracle_feed = market_params.oracle_feed;
        self.result = MarketOutcome::NotYetCommenced;
        self.start_time = market_params.start_time;
        self.end_time = market_params.end_time;
        self.is_settled = false;
        self.house_pool = house_pool;
        self.market_authority = market_authority;
        self.creation_time = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}
    #[error_code]
pub enum ErrorCode {
    InvalidOutcomesCount,
}