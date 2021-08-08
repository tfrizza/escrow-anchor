use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Escrow {
    pub is_initialized: bool,
    pub initializer: Pubkey,
    pub escrow_token_account: Pubkey,
    pub expected_amount: u64,
}