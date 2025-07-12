use anchor_lang::prelude::*;

declare_id!("7tksPo2em6C4DgFFitCVh296SFZ4JeAE1TK2vKbfauJB");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod hello_anchor_6 {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}
