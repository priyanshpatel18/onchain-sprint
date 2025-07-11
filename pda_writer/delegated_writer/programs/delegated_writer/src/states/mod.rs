use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
  pub authority: Pubkey,
  pub bump: u8,
  #[max_len(32)]
  pub name: String
}