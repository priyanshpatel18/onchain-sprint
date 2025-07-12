use anchor_lang::prelude::*;

use crate::states::Bank;

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Vault
    #[account(
    mut,
    seeds = [b"vault", user.key().as_ref()],
    bump
  )]
    pub vault: AccountInfo<'info>,

    #[account(
    init,
    payer = user,
    space = 8 + Bank::INIT_SPACE,
    seeds = [b"bank", user.key().as_ref()],
    bump
  )]
    pub bank: Account<'info, Bank>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitBank<'info> {
    pub fn init_bank(&mut self, bumps: InitBankBumps) -> Result<()> {
        self.bank.set_inner(Bank {
            authority: self.user.key(),
            amount: 0,
            bank_bump: bumps.bank,
            vault_bump: bumps.vault,
        });
        Ok(())
    }
}
