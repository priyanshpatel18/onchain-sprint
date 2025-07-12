use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::states::Vault;

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      mint::decimals = 6,
      mint::authority = vault
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
      mut,
      associated_token::mint = mint,
      associated_token::authority = vault
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitVault<'info> {
    pub fn init_vault(&mut self, amount: u64, bumps: InitVaultBumps) -> Result<()> {
        let user_key = self.user.key();
        let signer_seeds: &[&[u8]] = &[b"vault", user_key.as_ref(), &[bumps.vault]];
        let signer_seeds_slice = &[signer_seeds];

        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds_slice,
        );

        self.vault.set_inner(Vault {
            authority: self.user.key(),
            amount,
            vault_bump: bumps.vault,
        });

        mint_to(cpi_ctx, amount)
    }
}
