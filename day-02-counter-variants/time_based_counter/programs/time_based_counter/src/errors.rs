use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Too soon, wait 10 seconds.")]
    CooldownNotMet,
}
