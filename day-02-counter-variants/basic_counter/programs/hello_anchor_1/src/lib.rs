use anchor_lang::prelude::*;

declare_id!("8E74aT36YCrbwidsxzmBNuV4ocHFBtTR7RXsucyvFpvx");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod basic_counter {
    use super::*;

    pub fn init_count(ctx: Context<InitCount>) -> Result<()> {
        ctx.accounts.init_count()?;
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()?;
        Ok(())
    }
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        ctx.accounts.decrement()?;
        Ok(())
    }
}