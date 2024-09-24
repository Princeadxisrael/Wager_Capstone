
// use anchor_lang::prelude::*;
// use anchor_lang::solana_program::{
//     sysvar::instructions::{ID as INSTRUCTIONS_ID, load_instruction_at_checked},
//     ed25519_program::ID as ED25519_PROGRAM_ID,
// };
// use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
// use crate::state::{Event, Vault, Bet, LiquidityPool};
// use crate::errors::ErrorCode;

// #[derive(Accounts)]
// pub struct ResolveEvent<'info> {
//     #[account(mut)]
//     pub event: Account<'info, Event>,
//     #[account(
//         mut,
//         seeds = [b"vault", event.key().as_ref()],
//         bump
//     )]
//     pub vault: Account<'info, Vault>,
//     #[account(mut)]
//     pub resolver: Signer<'info>,
//     #[account(
//         mut,
//         seeds = [b"liquidity_pool", event.creator.as_ref()],
//         bump
//     )]
//     pub liquidity_pool: Account<'info, LiquidityPool>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     /// CHECK:
//     #[account(address = ED25519_PROGRAM_ID)]
//     pub ed25519_program: AccountInfo<'info>,
//     /// CHECK: 
//     #[account(address = INSTRUCTIONS_ID)]
//     pub instructions_sysvar: AccountInfo<'info>,
// }

// impl<'info> ResolveEvent<'info> {
//     pub fn resolve_event(ctx: Context<ResolveEvent>, winning_outcome: u8) -> Result<()> {
//         let event = &mut ctx.accounts.event;
//         let vault = &mut ctx.accounts.vault;
//         let liquidity_pool = &mut ctx.accounts.liquidity_pool;

//         // Verify the Ed25519 signature
//         let ix = load_instruction_at_checked(0, &ctx.accounts.instructions_sysvar)?;
        
//         // Ensure this instruction is being called directly and not via CPI
//         require!(
//             ix.program_id == ctx.program_id,
//             ErrorCode::UnauthorizedResolution
//         );

//         // Extract signature and public key from instruction data
//         require!(ix.data.len() >= 96, ErrorCode::InvalidInstructionData);
//         let signature = &ix.data[..64];
//         let pubkey = &ix.data[64..96];

//         // Construct the message that was signed
//         let event_pubkey = ctx.accounts.event.key();
//         let message = [&event_pubkey.to_bytes()[..], &[winning_outcome]].concat();

//         // Verify the Ed25519 signature
//         let ed25519_ix = Instruction::new_with_bytes(
//             ED25519_PROGRAM_ID,
//             signature,
//             vec![
//                 ed25519_program::Ed25519ProgramAccount {
//                     pubkey: pubkey.try_into().unwrap(),
//                     message: message.into(),
//                 }.try_to_vec()?,
//             ],
//         );

//         solana_program::program::invoke(
//             &ed25519_ix,
//             &[ctx.accounts.ed25519_program.to_account_info()]
//         )?;

//         // Proceed with event resolution if signature is valid
//         let event = &mut ctx.accounts.event;
//         let vault = &mut ctx.accounts.vault;

//         // Check if the event is still active
//         require!(event.is_active, ErrorCode::EventAlreadyResolved);

//         // Check if the winning outcome is valid
//         require!(
//             winning_outcome < event.outcomes.len() as u8,
//             ErrorCode::InvalidWinningOutcome
//         );

//         // Calculate total payout
//         let total_payout = Self::calculate_total_payout(event, winning_outcome)?;

//         // Check if the vault has sufficient balance
//         require!(
//             vault.balance >= total_payout,
//             ErrorCode::InsufficientVaultBalance
//         );

//         // Update event state
//         event.is_active = false;
//         event.winning_outcome = Some(winning_outcome);

//         // Process payouts
//         Self::process_payouts(ctx, event, vault, liquidity_pool, winning_outcome, total_payout)?;

//         Ok(())
//     }

//     fn calculate_total_payout(event: &Event, winning_outcome: u8) -> Result<u64> {
//         let mut total_payout = 0;
//         for bet in &event.bets {
//             if bet.outcome_index == winning_outcome {
//                 total_payout = total_payout
//                     .checked_add(bet.payout)
//                     .ok_or(ErrorCode::PayoutOverflow)?;
//             }
//         }
//         Ok(total_payout)
//     }

//     fn process_payouts(
//         ctx: Context<ResolveEvent>,
//         event: &mut Event,
//         vault: &mut Account<Vault>,
//         liquidity_pool: &mut Account<LiquidityPool>,
//         winning_outcome: u8,
//         total_payout: u64,
//     ) -> Result<()> {
//         let initial_liquidity = liquidity_pool.total_liquidity;

//         for bet in &mut event.bets {
//             if bet.outcome_index == winning_outcome {
//                 let payout_amount = bet.payout;
                
//                 // Check if bet is already resolved
//                 require!(!bet.is_resolved, ErrorCode::BetAlreadyResolved);
                
//                 // Transfer tokens from liquidity pool to bettor
//                 let cpi_accounts = Transfer {
//                     from: ctx.accounts.liquidity_pool.to_account_info(),
//                     to: bet.bettor.to_account_info(),
//                     authority: ctx.accounts.liquidity_pool.to_account_info(),
//                 };
//                 let cpi_program = ctx.accounts.token_program.to_account_info();
//                 let seeds = &[
//                     b"liquidity_pool",
//                     event.creator.as_ref(),
//                     &[ctx.bumps.liquidity_pool],
//                 ];
//                 let signer_seeds = &[&seeds[..]];

//                 token::transfer(
//                     CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
//                     payout_amount,
//                 )?;

//                 // Update liquidity pool balance
//                 liquidity_pool.total_liquidity = liquidity_pool.total_liquidity
//                     .checked_sub(payout_amount)
//                     .ok_or(ErrorCode::InsufficientLiquidity)?;

//                 // Mark bet as resolved
//                 bet.is_resolved = true;
//             }
//         }

//         // Ensure all payouts were processed correctly
//         require!(
//             liquidity_pool.total_liquidity == initial_liquidity - total_payout,
//             ErrorCode::PayoutMismatch
//         );

//         // Transfer remaining balance from vault to liquidity pool
//         let remaining_balance = vault.balance;
//         let cpi_accounts = Transfer {
//             from: ctx.accounts.vault.to_account_info(),
//             to: ctx.accounts.liquidity_pool.to_account_info(),
//             authority: ctx.accounts.vault.to_account_info(),
//         };
//         let cpi_program = ctx.accounts.token_program.to_account_info();
//         let signer_seeds = &[
//             b"vault",
//             event.key().as_ref(),
//             &[ctx.bumps.vault],
//         ];
//         let signer_seeds = &[&signer_seeds[..]];

//         token::transfer(
//             CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
//             remaining_balance,
//         )?;

//         // Update vault and liquidity pool balances
//         vault.balance = 0;
//         liquidity_pool.total_liquidity = liquidity_pool.total_liquidity
//             .checked_add(remaining_balance)
//             .ok_or(ErrorCode::PayoutOverflow)?;

//         Ok(())
//     }
// }