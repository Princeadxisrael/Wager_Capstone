use anchor_lang::prelude::*;
use crate::state::{Event, LiquidityPool, Market, MarketParams, Vault};

#[derive(Accounts)]
#[instruction(event_id: u64, market_id: u64)]
pub struct CreateEvent<'info> {
    //intialize new event 
    #[account(
        init,
        payer = creator,
        space = 8 + Event::INIT_SPACE,
        seeds = [b"event", event_id.to_le_bytes().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    //intialize new vault
    #[account(
        init,
        payer = creator,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", event.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    //initialize new market account
    #[account(
        init,
        payer = creator,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"market", market_id.to_le_bytes().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub creator: Signer<'info>, //event creator wallet as a signer is sufficient
    #[account(
        mut,
        seeds = [b"liquidity_pool", creator.key().as_ref()],
        bump
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
  
    #[account(mut)]
    pub market_authority: AccountInfo<'info>,

    #[account(mut)]
    pub house_pool: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateEvent<'info> {
    pub fn create_event(
        &mut self,
        event_id: u64,
        house_pool:Pubkey,
        market_params: MarketParams,
    ) -> Result<()> {
        // Set up the event
        
        // self.event.set_data(event_id, market_params.possible_outcomes.clone(), market_params.odds.clone(),self.creator.key())?;
        
        //Scenerio: Event::set_data and Market::set_data need to own the data
        let MarketParams {
            possible_outcomes,
            odds,
            ..
        } = market_params.clone();

        // Set up the event
        self.event.set_data(
            event_id,
            possible_outcomes,
            odds,
            self.creator.key()
        )?;

        // Set up the market
          self.market.set_data(
            market_params,
            self.creator.key(),
            house_pool
        )?;

        // Set up the vault
        self.vault.set_data(self.event.key())?;
        // Add event to pool
        self.liquidity_pool.add_event(self.event.key())?;

        Ok(())
    }
}



