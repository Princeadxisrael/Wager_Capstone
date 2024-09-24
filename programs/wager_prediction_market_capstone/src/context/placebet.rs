use anchor_lang::prelude::*;
use anchor_spl::token::{
    transfer, 
    Token, 
    TokenAccount, 
    Transfer
};

use crate::state::{Bet, Event, User, Vault, LiquidityPool, Market};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        init,
        payer = bettor,
        space = Bet::INIT_SPACE,
        seeds = [b"bet", event.key().as_ref(), bettor.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(
        mut,
        seeds = [b"vault", event.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub bettor: Signer<'info>,
    #[account(
        mut,
        constraint = bettor_token_account.owner == bettor.key()
    )]
    pub bettor_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"vault", event.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"user", bettor.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    #[account(
        mut,
        seeds = [b"market", bettor.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"liquidity_pool", event.creator.as_ref()],
        bump
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(
        mut,
        seeds = [b"house_pool"],
        bump
    )]
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(
        &mut self,
        outcome: u8,
        amount: u64,
        bump: u8,
    ) -> Result<()> {

        // Check if the event is active
        require!(self.event.is_active, ErrorCode::EventInactive);

        // Check if the outcome index is valid
        require!(
            outcome < self.event.outcomes.len() as u8,
            ErrorCode::InvalidOutcomeIndex
        );

        // Check if the amount is not too big (can be parsed to u32)
        require!(
            amount <= u32::MAX as u64,
            ErrorCode::AmountTooBig
        );

        // Check if the liquidity pool has enough liquidity
        require!(
            self.liquidity_pool.total_liquidity >= amount,
            ErrorCode::InsufficientLiquidity
        );

        // Calculate new odds based on liquidity pool
        let new_odds = self.calculate_odds(outcome, amount)?;

        self.bet.bettor = self.user;
        self.bet.market = self.market;
        self.bet.outcome= outcome;
        self.bet.amount = amount;
        self.bet.odds = new_odds;
        self.bet.timestamp = Clock::get()?.unix_timestamp;
        self.bet.settled = false;
        self.bet.user_payout = 0;

        self.event.total_bets[outcome as usize] = self.event.total_bets[outcome as usize]
            .checked_add(amount)
            .ok_or(ErrorCode::PayoutOverflow)?;

        //Update event odds
        self.event.odds[outcome as usize] = new_odds;

        self.user.total_bets = self.user.total_bets
            .checked_add(1)
            .ok_or(ErrorCode::PayoutOverflow)?;
        self.user.active_bets.push(self.bet.key());

        self.liquidity_pool.total_liquidity = self.liquidity_pool.total_liquidity
            .checked_sub(amount)
            .ok_or(ErrorCode::InsufficientLiquidity)?;

        let vault = &mut self.vault;
        vault.balance = vault.balance
            .checked_add(amount)
            .ok_or(ErrorCode::PayoutOverflow)?;
        
        // Transfer tokens from bettor to vault
        let cpi_accounts = Transfer {
            from: self.bettor_token_account.to_account_info(),
            to: self.vault_token_account.to_account_info(),
            authority: self.bettor.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    fn calculate_odds(&self, outcome: u8, bet_amount: u64) -> Result<u64> {
        let event = &self.event;
        let liquidity_pool = &self.liquidity_pool;

        let total_bets: u64 = event.total_bets.iter().sum();
        let new_total_bets = total_bets.checked_add(bet_amount).ok_or(ErrorCode::PayoutOverflow)?;

        let outcome_bets = event.total_bets[outcome as usize];
        let new_outcome_bets = outcome_bets.checked_add(bet_amount).ok_or(ErrorCode::PayoutOverflow)?;

        // Calculate the new odds based on the proportion of bets on this outcome
        let new_odds = (new_total_bets as f64 / new_outcome_bets as f64 * 100.0) as u64;

        // Apply a small adjustment based on the liquidity pool size
        let liquidity_factor = 1.0 + (liquidity_pool.total_liquidity as f64 / 1_000_000.0); // Adjust this factor as needed
        let adjusted_odds = (new_odds as f64 * liquidity_factor) as u64;

        Ok(adjusted_odds)
    }
}
