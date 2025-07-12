use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub authority: Pubkey,
    pub balance: u64,
    pub vault_bump: u8,
    pub bank_bump: u8,
}
