use anchor_lang::prelude::*;

use crate::states::UserProfile;

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      init,
      payer = user,
      seeds = [b"delegated_writer", user.key().as_ref()],
      bump,
      space = 8 + UserProfile::INIT_SPACE,
    )]
    pub profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitProfile<'info> {
    pub fn init_profile(&mut self, name: String, bumps: InitProfileBumps) -> Result<()> {
        self.profile.set_inner(UserProfile {
            authority: self.user.key(),
            bump: bumps.profile,
            name,
        });
        Ok(())
    }
}
