use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Post {
  pub index: u64,
  pub bump: u8,
  #[max_len(100)]
  pub data: String,
}