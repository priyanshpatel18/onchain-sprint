use anchor_lang::prelude::*;

declare_id!("26iMNSwrjEjLRF5D9TyucfmHhjZz2HhLChfzAydXLzGe");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod hello_anchor_5 {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}
