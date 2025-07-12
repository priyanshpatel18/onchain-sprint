use anchor_lang::prelude::*;

use crate::states::Profile;

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    space = 8 + Profile::INIT_SPACE,
    seeds = [b"nested", user.key().as_ref()],
    bump
  )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProfile<'info> {
    pub fn init_profile(&mut self, name: String, bumps: InitializeProfileBumps) -> Result<()> {
        self.profile.set_inner(Profile {
            bump: bumps.profile,
            authority: self.user.key(),
            name,
        });
        Ok(())
    }
}
