use anchor_lang::prelude::*;

use crate::errors::CustomError;
use crate::states::Count;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
      mut,
      seeds = [b"count"],
      bump
    )]
    pub count: Account<'info, Count>,
}

impl Increment<'_> {
    pub fn increment(&mut self) -> Result<()> {
      let count_account = &mut self.count;
      require!(
        count_account.count < 3,
        CustomError::CountOverflow
      );

      count_account.count += 1;
      Ok(())
    }
}