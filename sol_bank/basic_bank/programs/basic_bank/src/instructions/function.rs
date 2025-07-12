use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke},
        system_instruction::transfer,
    },
};

use crate::{errors::ErrorCode, states::Bank};

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

    pub system_program: Program<'info, System>,
}

impl<'info> Function<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let ix = transfer(&self.user.key(), &self.vault.key(), amount);

        invoke(
            &ix,
            &[self.user.to_account_info(), self.vault.to_account_info()],
        )?;

        self.bank.balance += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(self.bank.balance >= amount, ErrorCode::InvalidAmount);

        // Transfer SOL directly by manipulating lamports
        **self.vault.try_borrow_mut_lamports()? -= amount;
        **self.user.to_account_info().try_borrow_mut_lamports()? += amount;

        self.bank.balance -= amount;

        Ok(())
    }
}
