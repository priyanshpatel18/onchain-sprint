use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Count {
    pub authority: Pubkey,
    pub count: u64,
}
