use anchor_lang::prelude::*;

declare_id!("86Uce9bTCk2uo8DfjMyvj1R3f9njWhSfWVySDS8epP8S");

pub mod state;
pub mod context;
pub mod errors;

pub use context::*;
pub use state::*;

#[program]
pub mod wager_prediction_market_capstone {
    

    use state::MarketParams;

    use super::*;

    pub fn createevent(ctx: Context<CreateEvent>,event_id:u64, house_pool:Pubkey, market_params: MarketParams ) -> Result<()> {
        ctx.accounts.create_event(event_id, house_pool, market_params)?;
        Ok(())
    }
    pub fn placebet(ctx: Context<PlaceBet>, outcome:u8, amount:u64,bet_amount:u64) -> Result<()> {
        ctx.accounts.place_bet( outcome, amount)?;
        ctx.accounts.calculate_odds(outcome, bet_amount)?;
        Ok(())
    }
    pub fn resolveevent(ctx: Context<ResolveEvent>, winning_outcome:u8) -> Result<()> {
        ctx.accounts.resolve_event(
          
            winning_outcome)?;
       Ok(())
     
    }
}


