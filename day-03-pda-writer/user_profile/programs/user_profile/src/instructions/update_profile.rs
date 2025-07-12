use anchor_lang::prelude::*;

use crate::states::UserProfile;

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: authority account to be passed by client
    pub authority: AccountInfo<'info>,

    #[account(
    mut,
    seeds = [b"profile", user.key().as_ref()],
    bump,
    has_one = authority
  )]
    pub profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> UpdateProfile<'_> {
    pub fn update_profile(
        &mut self,
        username: String,
        bio: String,
        bumps: UpdateProfileBumps,
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
