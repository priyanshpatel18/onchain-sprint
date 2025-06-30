use anchor_lang::prelude::*;

declare_id!("9cYUQsghBj1JnWNVJmGEmvHBREHBopXfexWKYvqMaUR");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod hello_anchor_3 {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
    
}