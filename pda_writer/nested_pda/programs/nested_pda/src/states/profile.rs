use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
  pub bump: u8,
  pub authority: Pubkey,
  #[max_len(32)]
  pub name: String,
}