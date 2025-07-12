use anchor_lang::prelude::*;

declare_id!("FyzGH1zRNNWVDEw1a2PJoS7zQCAW3QToxtaJbFkM8AGz");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod basic_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitializeBank>) -> Result<()> {
        ctx.accounts.init_bank(ctx.bumps)
    }

    pub fn deposit(ctx: Context<Function>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Function>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}
