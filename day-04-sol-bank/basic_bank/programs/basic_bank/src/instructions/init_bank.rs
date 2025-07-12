use anchor_lang::prelude::*;

use crate::states::Bank;

#[derive(Accounts)]
pub struct InitializeBank<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Vault
    #[account(
      init,
      payer = user,
      space = 8 + 32,
      seeds = [b"vault", user.key().as_ref()],
      bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(
      init,
      seeds = [b"bank", user.key().as_ref()],
      bump,
      payer = user,
      space = 8 + Bank::INIT_SPACE
    )]
    pub bank: Account<'info, Bank>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeBank<'info> {
    pub fn init_bank(&mut self, bumps: InitializeBankBumps) -> Result<()> {
        self.bank.set_inner(Bank {
            authority: self.user.key(),
            balance: 0,
            vault_bump: bumps.vault,
            bank_bump: bumps.bank,
        });
        Ok(())
    }
}
