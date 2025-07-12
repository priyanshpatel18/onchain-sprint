use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
  pub initiator: Pubkey,
  pub receiver: Pubkey,
  pub amount: u64,
  pub bump: u8
}