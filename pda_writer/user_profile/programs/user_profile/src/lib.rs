use anchor_lang::prelude::*;

declare_id!("AAJE7GTaZzPz6oP35SUCLpqWQwpjzwTR5YjpFN7XcSvR");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod user_profile {
    use super::*;

    pub fn init_profile(ctx: Context<InitializeProfile>, username: String, bio: String) -> Result<()> {
        ctx.accounts.init_profile(username, bio, ctx.bumps)?;
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, username: String, bio: String) -> Result<()> {
        ctx.accounts.update_profile(username, bio, ctx.bumps)?;
        Ok(())
    }
}