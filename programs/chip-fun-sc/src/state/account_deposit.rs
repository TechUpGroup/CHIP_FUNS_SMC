use anchor_lang::prelude::*;

#[account]
pub struct AccountDeposit {
    pub owner: Pubkey,
    pub deposit_amount: u64,
    pub withdraw_amount: u64,
}

impl AccountDeposit {
    pub const LEN: usize = 8 + 1 * 32 + 8 * 2;
}
