use anchor_lang::prelude::*;

use crate::state::{Market, User};


#[account]
#[derive(InitSpace)]
pub struct  Bet {
    pub bettor: User,
    pub market: Market,
    pub user_usdt_account: Pubkey,
    pub user_main_account: Pubkey,
    pub amount:u64,
    pub user_risk: u64,
    pub user_payout: u64,
    pub points: u16,
    pub user_market_side: u8,
    pub outcome: u8,
    pub bump:u8,
    pub timestamp: i64,
    pub odds: u64,
    pub settled: bool,
    pub cancelled: bool,
}
