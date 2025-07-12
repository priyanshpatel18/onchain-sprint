use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Invalid amount")]
  InvalidAmount,
  #[msg("Invalid initiator")]
  InvalidInitiator,
  #[msg("Invalid receiver")]
  InvalidReceiver,
}