use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{errors::CustomError, states::Vault};

#[derive(Accounts)]
pub struct SendToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
      mint::decimals = 6,
      mint::authority = vault
    )]
    pub mint: Account<'info, Mint>,

    #[account(
      mut,
      associated_token::mint = mint,
      associated_token::authority = vault
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
      init_if_needed,
      payer = user,
      associated_token::mint = mint,
      associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> SendToken<'info> {
    pub fn send_token(&mut self, amount: u64, bumps: SendTokenBumps) -> Result<()> {
        require!(
            amount <= self.vault.amount,  
            CustomError::InsufficientBalance
        );

        let accounts = Transfer {
            from: self.vault_ata.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let user_key = self.user.key();
        let seeds: &[&[u8]] = &[b"vault", user_key.as_ref(), &[bumps.vault]];
        let signer_seeds = &[seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(cpi_ctx, amount)
    }
}
