use anchor_lang::prelude::*;

declare_id!("5AG4bmbmfn1J7c9mr5vPZ6jVb2E8CNb3U34JWRGQ7Rw3");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod hello_anchor_1 {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}