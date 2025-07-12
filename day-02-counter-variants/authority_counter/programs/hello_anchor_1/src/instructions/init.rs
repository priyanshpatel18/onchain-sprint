use anchor_lang::prelude::*;

use crate::states::Count;

#[derive(Accounts)]
pub struct InitCount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
      init_if_needed,
      payer= signer,
      space = 8 + Count::INIT_SPACE,
      seeds = [b"count"],
      bump
    )]
    pub count: Account<'info, Count>,

    pub system_program: Program<'info, System>
}

impl InitCount<'_> {
    pub fn init_count(&mut self) -> Result<()> {
      let count_account = &mut self.count;
      count_account.count = 0;
      count_account.authority = self.signer.key();
      Ok(())
    }
}