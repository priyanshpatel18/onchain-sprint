use anchor_lang::prelude::*;

use crate::states::Count;
use crate::errors::MyError;

#[derive(Accounts)]
pub struct  Increment<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    mut,
    seeds = [b"count"],
    bump   
  )]
  pub count: Account<'info, Count>
}

impl Increment<'_> {
    pub fn increment(&mut self) -> Result<()> {
      let count_account = &mut  self.count;
      let clock = Clock::get()?;
      let timestamp = clock.unix_timestamp;

      require!(
        timestamp - count_account.timestamp >= 10,
         MyError::CooldownNotMet
      );
      count_account.count += 1;
      count_account.timestamp = timestamp;
      Ok(())
    }
}