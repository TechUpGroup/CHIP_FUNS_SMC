use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{error::ErrorMessage, AccountDeposit, Vault};
use crate::utils::transfer_token_to_account;
#[derive(Accounts)]
#[instruction(amount: u64, id: String)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(
        mut,
         associated_token::mint = mint,
         associated_token::authority = depositor,
         constraint = ata_depositor.amount >= amount @ErrorMessage::BalanceInsufficient
     )]
     pub ata_depositor: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = depositor,
        space = AccountDeposit::LEN,
        seeds = [
            b"AccountDeposit", mint.key().as_ref(),id.as_bytes().as_ref()
        ],
        bump
    )]
    pub account_deposit: Box<Account<'info,AccountDeposit>>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"Vault"],
        bump,
        has_one = mint
    )]
    pub vault: Box<Account<'info, Vault>>,
    #[account(
       mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
    )]
    pub ata_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler_deposit(ctx: Context<Deposit>,amount:u64, _id: String) -> Result<()> {
    transfer_token_to_account(
        ctx.accounts.ata_depositor.to_account_info(),
        ctx.accounts.ata_vault.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        None,
    )?;

    let account_deposit = &mut ctx.accounts.account_deposit;
    account_deposit.deposit_amount = account_deposit.deposit_amount.checked_add(amount).unwrap_or(0);
    
    Ok(())
}
