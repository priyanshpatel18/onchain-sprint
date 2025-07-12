use anchor_lang::prelude::*;

use crate::states::{ Count};

#[derive(Accounts)]
pub struct Increment<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  /// CHECK: Signer
  pub authority: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [b"count"],
    bump,
    has_one = authority
  )]
  pub count: Account<'info, Count>,

  pub system_program: Program<'info, System>
}

impl Increment<'_> {
    pub fn increment(&mut self) -> Result<()> {
      let count_account = &mut self.count;
      count_account.count += 1;
      Ok(())
    }
}