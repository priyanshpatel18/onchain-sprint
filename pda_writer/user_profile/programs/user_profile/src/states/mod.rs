use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
  pub authority: Pubkey,
  #[max_len(32)]
  pub username: String,
  #[max_len(120)]
  pub bio: String,
  pub bump: u8
}