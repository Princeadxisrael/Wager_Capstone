use anchor_lang::prelude::*;

declare_id!("86Uce9bTCk2uo8DfjMyvj1R3f9njWhSfWVySDS8epP8S");

#[program]
pub mod wager_prediction_market_capstone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
