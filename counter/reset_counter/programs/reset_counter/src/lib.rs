use anchor_lang::prelude::*;

declare_id!("CsDLE3sFogoRrtdR3Z4t9JNAUu6XNReE5zHAcQanXa19");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod reset_counter {
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
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        ctx.accounts.reset()?;
        Ok(())
    }
}