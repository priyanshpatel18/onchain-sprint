use anchor_lang::prelude::*;

declare_id!("CW7q1tvL3hM9Jwtd6j1NFJgFMqmPS2b8U4sZJjR5YpF5");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod p2p_transfer {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(amount, ctx.bumps)
    }

    pub fn accept(ctx: Context<Accept>) -> Result<()> {
        ctx.accounts.accept_and_close(ctx.bumps)
    }
}
