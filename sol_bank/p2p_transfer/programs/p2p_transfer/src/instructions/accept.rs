use anchor_lang::{prelude::*, solana_program::{program::invoke_signed, system_instruction::transfer}};

use crate::{errors::ErrorCode, states::Escrow};
#[derive(Accounts)]
pub struct Accept<'info> {
    /// CHECK: not signer, but verified via seeds
    #[account(mut)]
    pub initiator: AccountInfo<'info>,

    #[account(mut)]
    pub receiver: Signer<'info>,

    /// CHECK: we verify vault manually via seeds
    #[account(
        mut,
        seeds = [b"vault", initiator.key().as_ref(), receiver.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"escrow", initiator.key().as_ref(), receiver.key().as_ref()],
        bump,
        close = receiver
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

impl<'info> Accept<'info> {
    pub fn accept_and_close(&mut self, bumps: AcceptBumps) -> Result<()> {
        require!(self.escrow.initiator == self.initiator.key(), ErrorCode::InvalidInitiator);
        require!(self.escrow.receiver == self.receiver.key(), ErrorCode::InvalidReceiver);
        require!(self.escrow.amount > 0, ErrorCode::InvalidAmount);

        let transfer_ix = transfer(
            &self.vault.key(),
            &self.receiver.key(),
            self.escrow.amount,
        );

        invoke_signed(
            &transfer_ix,
            &[
                self.vault.to_account_info(),
                self.receiver.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[
                b"vault",
                self.initiator.key.as_ref(),
                self.receiver.key.as_ref(),
                &[bumps.vault],
            ]],
        )?;

        Ok(())
    }
}
