use anchor_lang::prelude::*;

use crate::states::Count;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
      mut,
      seeds = [b"count"],
      bump
    )]
    pub count: Account<'info, Count>,
}

impl Decrement<'_> {
    pub fn decrement(&mut self) -> Result<()> {
      let count_account = &mut self.count;
      count_account.count -= 1;
      Ok(())
    }
}