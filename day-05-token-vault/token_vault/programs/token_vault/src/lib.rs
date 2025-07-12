use anchor_lang::prelude::*;

declare_id!("9mbzPokq5FtDjyupXp7sqn8PfpcQiz3nshUAQjvH8EcP");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod token_vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, amount: u64) -> Result<()> {
        ctx.accounts.init_vault(amount, ctx.bumps)
    }
    pub fn send_tokens(ctx: Context<SendToken>, amount: u64) -> Result<()> {
        ctx.accounts.send_token(amount, ctx.bumps)
    }
}