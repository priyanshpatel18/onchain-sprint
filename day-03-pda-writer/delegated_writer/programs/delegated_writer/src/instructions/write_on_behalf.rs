use anchor_lang::prelude::*;

use crate::states::UserProfile;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct WriteOnBehalf<'info> {
    /// CHECK: Only used for PDA derivation
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [b"delegated_writer", user.key().as_ref()],
      bump
    )]
    pub profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> WriteOnBehalf<'info> {
    pub fn write_on_behalf(&mut self, name: String, bumps: WriteOnBehalfBumps) -> Result<()> {
        require!(self.profile.authority == self.authority.key(), ErrorCode::InvalidAuthority);

        self.profile.set_inner(UserProfile {
            authority: self.authority.key(),
            bump: bumps.profile,
            name,
        });

        Ok(())
    }
}