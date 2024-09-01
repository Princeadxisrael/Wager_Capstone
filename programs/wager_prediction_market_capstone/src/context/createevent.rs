use anchor_lang::prelude::*;
use crate::state::Event;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = creator, space = Event::INIT_SPACE)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateEvent<'info> {
    pub fn create_event(
        &mut self,
        id: u64,
        outcomes: Vec<String>,
        odds: Vec<u64>,
    ) -> Result<()> {
        let event = &mut self.event;
        event.id = id;
        event.outcomes = outcomes;
        event.odds = odds.clone();
        event.is_active = true;
        event.total_bets = vec![0; odds.len()];
        event.winning_outcome = None; // Initialize with None, will be set when resolving the event

        Ok(())
    }
}





