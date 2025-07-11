
use anchor_lang::prelude::*;

use crate::states::UserProfile;

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    init,
    payer = user,
    space = 8 + UserProfile::INIT_SPACE,
    seeds = [b"profile", user.key().as_ref()],
    bump
  )]
    pub profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProfile<'_> {
    pub fn init_profile(
        &mut self,
        username: String,
        bio: String,
        bumps: InitializeProfileBumps,
    ) -> Result<()> {
        self.profile.set_inner(UserProfile {
            authority: self.user.key(),
            username,
            bio,
            bump: bumps.profile,
        });
        Ok(())
    }
}
