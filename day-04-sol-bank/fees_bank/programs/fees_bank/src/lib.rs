use anchor_lang::prelude::*;

declare_id!("2jDxJPpPdfTf5LLBw5d7yL6NvsTu3zZEqcUBHxppBpf3");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod fees_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>) -> Result<()> {
        ctx.accounts.init_bank(ctx.bumps)
    }

    pub fn deposit(ctx: Context<Function>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Function>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, ctx.bumps)
    }
}