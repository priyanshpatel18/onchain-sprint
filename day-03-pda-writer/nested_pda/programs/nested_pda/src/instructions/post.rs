use anchor_lang::prelude::*;

use crate::states::{Post, Profile};

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct PostFunction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,


    /// CHECK: Authority
    pub authority: AccountInfo<'info>,

    #[account(
        seeds = [b"nested", user.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub profile: Account<'info, Profile>,

    #[account(
      init,
      payer = user,
      space = 8 + Post::INIT_SPACE,
      seeds = [b"post", profile.key().as_ref(), index.to_le_bytes().as_ref()],
      bump
    )]
    pub post: Account<'info, Post>,

    pub system_program: Program<'info, System>,
}

impl<'info> PostFunction<'info> {
    pub fn post(&mut self, index: u64, data: String, bumps: PostFunctionBumps) -> Result<()> {
        self.post.set_inner(Post { index, bump: bumps.post, data });
        Ok(())
    }
}
