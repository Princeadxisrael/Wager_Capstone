
use anchor_lang::{prelude::*, system_program::{transfer, Transfer} };
use anchor_lang::solana_program::ed25519_program;
use anchor_lang::solana_program::{
    sysvar::instructions::{ID as INSTRUCTIONS_ID, load_instruction_at_checked},
    ed25519_program::ID as ED25519_PROGRAM_ID,
};
use crate::state::{Event, Vault, Bet, LiquidityPool};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(
        mut,
        seeds = [b"vault", event.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub resolver: Signer<'info>,
    #[account(mut)]
    pub bettor: SystemAccount<'info>,
    #[account(
        mut,
        close = bettor,
        seeds = [b"bet", event.key().as_ref(), bettor.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        mut,
        seeds = [b"liquidity_pool", event.creator.key().as_ref()],
        bump
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    /// CHECK:
    #[account(address = ED25519_PROGRAM_ID)]
    pub ed25519_program: AccountInfo<'info>,
    /// CHECK: 
    #[account(address = INSTRUCTIONS_ID)]
    pub instructions_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> ResolveEvent<'info> {
    pub fn resolve_event (&mut self, sig:&[u8], winning_outcome:u8) -> Result<()> {
        // Verify the Ed25519 signature
        let ix = load_instruction_at_checked(0, &self.instructions_sysvar)?;
        
        require_keys_eq!(ix.program_id, ed25519_program::ID, ErrorCode::UnauthorizedResolution);
        
        // Extract signature and public key from instruction data
        require!(ix.data.len() >= 96, ErrorCode::InvalidInstructionData);
        let signature = &ix.data[..64];
        let pubkey = &ix.data[64..96];

        // Construct the message that was signed
        let event_pubkey = self.event.key();
        let message = [&event_pubkey.to_bytes()[..], &[winning_outcome]].concat();

       
        // Proceed with event resolution if signature is valid
        let event = &self.event;
        let vault = &self.vault;

        // Check if the event is still active
        require!(event.is_active, ErrorCode::EventAlreadyResolved);

        // Check if the winning outcome is valid
        require!(
            winning_outcome < event.outcomes.len() as u8,
            ErrorCode::InvalidWinningOutcome
        );

        // Calculate total payout
        let total_payout = Self::calculate_total_payout(event, winning_outcome)?;

        // Check if the vault has sufficient balance
        require!(
            vault.balance >= total_payout,
            ErrorCode::InsufficientVaultBalance
        );

        // Update event state
        event.is_active = false;
        event.winning_outcome = winning_outcome;

        // Process payouts
        self.process_payouts(&mut self.bet,  &mut &event, &mut &vault, &mut self.liquidity_pool, winning_outcome, total_payout, bumps)?;

        Ok(())
    }

    fn calculate_total_payout(event: &Event, winning_outcome: u8) -> Result<u64> {
        let mut total_payout: u64 = 0;
       
            if event.winning_outcome == winning_outcome {
                total_payout = total_payout
                    .checked_add(event.total_bets.iter().sum())
                    .ok_or(ErrorCode::PayoutOverflow)?;
            }
       
        Ok(total_payout as u64)
    }

    fn process_payouts(
        &mut self,
        bet: &mut Bet,
        event: &mut Event,
        vault: &mut Account<Vault>,
        liquidity_pool: &mut Account<LiquidityPool>,
        winning_outcome: u8,
        total_payout: u64,
        bumps: &ResolveEventBumps
    ) -> Result<()> {
        let initial_liquidity = liquidity_pool.total_liquidity;
            
            if bet.outcome == winning_outcome {
                let payout_amount = bet.user_payout;
                
                // Check if bet is already resolved
                require!(!bet.settled, ErrorCode::BetAlreadyResolved);
                
                // Transfer tokens from liquidity pool to bettor
                let cpi_accounts = Transfer {
                    from: self.liquidity_pool.to_account_info(),
                    to: self.bettor.to_account_info(),

                };
                let cpi_program = self.system_program.to_account_info();
                let seeds = &[
                    b"liquidity_pool",
                    self.event.creator.to_account_info().key().as_ref(),
                    &[self.liquidity_pool.bump],
                ];
                let signer_seeds = &[&seeds[..]];

               transfer(
                    CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
                    payout_amount,
                )?;

                // Update liquidity pool balance
                liquidity_pool.total_liquidity = liquidity_pool.total_liquidity
                    .checked_sub(payout_amount)
                    .ok_or(ErrorCode::InsufficientLiquidity)?;

                // Mark bet as resolved
                bet.settled = true;
            
        }

        // Ensure all payouts were processed correctly
        require!(
            liquidity_pool.total_liquidity == initial_liquidity - total_payout,
            ErrorCode::PayoutMismatch
        );

        // Transfer remaining balance from vault to liquidity pool
        let remaining_balance = vault.balance;
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.liquidity_pool.to_account_info(),
           
        };
        let cpi_program = self.system_program.to_account_info();
        let signer_seeds = &[
            b"vault",
            event.key().as_ref(),
            &[self.bumps.vault],
        ];
        let signer_seeds = &[&signer_seeds[..]];

       transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
            remaining_balance,
        )?;

        // Update vault and liquidity pool balances
        vault.balance = 0;
        liquidity_pool.total_liquidity = liquidity_pool.total_liquidity
            .checked_add(remaining_balance)
            .ok_or(ErrorCode::PayoutOverflow)?;

        Ok(())
    }
}