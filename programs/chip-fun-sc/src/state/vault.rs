use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub operator: Pubkey,
    pub authority: Pubkey,
    pub mint: Pubkey
}

impl Vault {
    pub const LEN: usize = 8 + 3 * 32;
}
