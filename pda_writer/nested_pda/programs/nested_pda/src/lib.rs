use anchor_lang::prelude::*;

declare_id!("HUyZiDGuJ1bs8HEqP4jdnLcaSaoyEWWXcmqoTEw12orV");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod nested_pda {
    use super::*;

    pub fn init_profile(ctx: Context<InitializeProfile>, name: String) -> Result<()> {
        ctx.accounts.init_profile(name, ctx.bumps)?;
        Ok(())
    }

    pub fn post(ctx: Context<PostFunction>, index: u64, data: String) -> Result<()> {
        ctx.accounts.post(index, data, ctx.bumps)?;
        Ok(())
    }
}