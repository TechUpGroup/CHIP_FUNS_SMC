use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::utils::transfer_token_to_account;
use crate::{error::ErrorMessage, AccountDeposit, Vault};
#[derive(Accounts)]
#[instruction(amount: u64, id: String)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK only for pubKey
    pub receiver: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = sender,
         associated_token::mint = mint,
         associated_token::authority = receiver,
     )]
    pub ata_receiver: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [
            b"AccountDeposit", mint.key().as_ref(),id.as_bytes().as_ref()
        ],
        bump
    )]
    pub account_deposit: Box<Account<'info, AccountDeposit>>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"Vault"],
        bump,
        has_one = mint,
    )]
    pub vault: Box<Account<'info, Vault>>,
    #[account(
       mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
        constraint = ata_vault.amount > amount @ErrorMessage::BalanceInsufficient
    )]
    pub ata_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK only for pubKey
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler_withdraw(ctx: Context<Withdraw>, amount: u64, _id: String) -> Result<()> {
    require!(ctx.accounts.operator.key() == ctx.accounts.vault.operator, ErrorMessage::InvalidOperator);
    let seeds = &[&[b"Vault", bytemuck::bytes_of(&ctx.bumps.vault)][..]];

    transfer_token_to_account(
        ctx.accounts.ata_vault.to_account_info(),
        ctx.accounts.ata_receiver.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds),
    )?;

    let account_deposit = &mut ctx.accounts.account_deposit;
    account_deposit.withdraw_amount = account_deposit
        .withdraw_amount
        .checked_add(amount)
        .unwrap_or(0);

    Ok(())
}
