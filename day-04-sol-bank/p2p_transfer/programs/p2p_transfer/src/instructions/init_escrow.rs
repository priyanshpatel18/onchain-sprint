use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};

use crate::states::Escrow;

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    /// CHECK: Receiver
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    /// CHECK: Vault
    #[account(
      mut,
      seeds = [b"vault", initiator.key().as_ref(), receiver.key().as_ref()],
      bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(
    init,
    payer = initiator,
    space = 8 + Escrow::INIT_SPACE,
    seeds = [b"escrow", initiator.key().as_ref(), receiver.key().as_ref()],
    bump
  )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitEscrow<'info> {
    pub fn init_escrow(&mut self, amount: u64, bumps: InitEscrowBumps) -> Result<()> {
        let ix = transfer(
            self.initiator.to_account_info().key,
            self.vault.to_account_info().key,
            amount,
        );

        invoke(
            &ix,
            &[
                self.initiator.to_account_info(),
                self.vault.to_account_info(),
            ],
        )?;

        self.escrow.set_inner(Escrow {
            initiator: self.initiator.key(),
            receiver: self.receiver.key(),
            amount,
            bump: bumps.escrow,
        });

        Ok(())
    }
}
