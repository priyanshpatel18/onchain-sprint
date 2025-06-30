use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Count cannot be 5 or more")]
    CountOverflow,
}
