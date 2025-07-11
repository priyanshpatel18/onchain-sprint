use anchor_lang::prelude::*;

use crate::states::UserProfile;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct DelegateWriter<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [b"delegated_writer", user.key().as_ref()],
      bump
    )]
    pub profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> DelegateWriter<'info> {
    pub fn delegate_writer(&mut self, writer: Pubkey, bumps: DelegateWriterBumps) -> Result<()> {
        require!(self.profile.authority == self.user.key(), ErrorCode::InvalidAuthority);

        self.profile.set_inner(UserProfile {
            authority: writer,
            bump: bumps.profile,
            name: self.profile.name.clone(),
        });
        Ok(())
    }
}