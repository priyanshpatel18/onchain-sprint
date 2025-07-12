use anchor_lang::prelude::*;

declare_id!("3fpwiEB1qNXScR6gY6CgXTYXBaMN5zpnqZamJaGWSdM5");

pub mod instructions;
pub use instructions::*;

#[program]
pub mod hello_anchor_2 {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}