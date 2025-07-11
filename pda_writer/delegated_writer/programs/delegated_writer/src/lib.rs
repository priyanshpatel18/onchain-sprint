use anchor_lang::prelude::*;

declare_id!("BHzMR1c8nSeUnLgPkHmbJt4T2ZtYMjyTPpVasLa67nuC");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod delegated_writer {
    use super::*;

    pub fn init_profile(ctx: Context<InitProfile>, name: String) -> Result<()> {
        ctx.accounts.init_profile(name, ctx.bumps)?;
        Ok(())
    }

    pub fn delegate_writer(ctx: Context<DelegateWriter>, writer: Pubkey) -> Result<()> {
        ctx.accounts.delegate_writer(writer, ctx.bumps)?;
        Ok(())
    }

    pub fn write_on_behalf(ctx: Context<WriteOnBehalf>, name: String) -> Result<()> {
        ctx.accounts.write_on_behalf(name, ctx.bumps)?;
        Ok(())
    }
}
