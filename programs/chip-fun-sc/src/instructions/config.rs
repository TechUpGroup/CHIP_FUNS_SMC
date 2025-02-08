use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct Config<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"Vault"],
        bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
}

pub fn handle_update(
    ctx: Context<Config>,
    new_operator: Option<Pubkey>,
    new_authority: Option<Pubkey>,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    if let Some(new_operator) = new_operator {
        vault.operator = new_operator;
    }

    if let Some(new_authority) = new_authority {
        vault.operator = new_authority;
    }

    Ok(())
}
