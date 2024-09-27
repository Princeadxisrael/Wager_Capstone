use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub event: Pubkey,
    pub balance: u64,
    pub bump:u8
}

impl Vault {
    pub fn set_data(&mut self, event: Pubkey) -> Result<()> {
        self.event = event;
        self.balance = 0; // Initialize balance to 0

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        //ensuring there is no arithmetic overflow
        self.balance = self.balance.checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        Ok(())
    }
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        if amount > self.balance {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        self.balance = self.balance.checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    ArithmeticOverflow,
    #[msg("There are no sufficient funds in this vault")]
    InsufficientFunds,
}