use anchor_lang::prelude::*;

declare_id!("54HzsL89KijACuMrNngz59nfKhczdUXFfkqTaNDT2JfQ");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod hello_anchor_4 {
    use super::*;
    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}
