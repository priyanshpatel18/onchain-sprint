use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Count {
  pub count: u64,
}