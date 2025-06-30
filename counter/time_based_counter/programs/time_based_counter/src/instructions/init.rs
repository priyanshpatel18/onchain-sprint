use anchor_lang::prelude::*;

use crate::states::Count;

#[derive(Accounts)]
pub struct  InitCount<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    init,
    space = 8 + Count::INIT_SPACE,
    payer = signer,
    seeds = [b"count"],
    bump   
  )]
  pub count: Account<'info, Count>,

  pub system_program: Program<'info, System>
}

impl InitCount<'_> {
    pub fn init_count(&mut self) -> Result<()> {
      let count_account = &mut  self.count;

      count_account.count = 0;
      let clock = Clock::get()?;
      let timestamp = clock.unix_timestamp;
      
      count_account.timestamp = timestamp;
      Ok(())
    }
}