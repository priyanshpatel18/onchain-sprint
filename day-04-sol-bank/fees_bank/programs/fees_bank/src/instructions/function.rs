use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction::transfer,
    },
};

use crate::states::Bank;

#[derive(Accounts)]
pub struct Function<'info> {
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
      mut,
      seeds = [b"bank", user.key().as_ref()],
      bump
    )]
    pub bank: Account<'info, Bank>,

    /// CHECK: Treasury
    #[account(
      mut,
      seeds = [b"treasury"],
      bump
    )]
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Function<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let ix = transfer(
            self.user.to_account_info().key,
            self.vault.to_account_info().key,
            amount,
        );

        invoke(
            &ix,
            &[self.user.to_account_info(), self.vault.to_account_info()],
        )?;

        self.bank.amount += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64, bumps: FunctionBumps) -> Result<()> {
        let fees = amount * 5 / 100;

        // PDA seeds
        let seeds = &[b"vault", self.user.key.as_ref(), &[bumps.vault]];

        // Transfer fees to treasury
        let fee_ix = transfer(self.vault.key, self.treasury.key, fees);

        invoke_signed(
            &fee_ix,
            &[
                self.vault.to_account_info(),
                self.treasury.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // Transfer remaining amount to user
        let net_amount = amount - fees;

        let withdraw_ix = transfer(self.vault.key, self.user.key, net_amount);

        invoke_signed(
            &withdraw_ix,
            &[
                self.vault.to_account_info(),
                self.user.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // Update state
        self.bank.amount -= amount;

        Ok(())
    }
}
