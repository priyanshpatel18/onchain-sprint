use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
  pub authority: Pubkey,
  pub amount: u64,
  pub bank_bump: u8,
  pub vault_bump: u8
}