pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use event::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("9RajujsLfxkrWmqb47YJVyjyJNtMsMjMaGwXTeQTbnq4");

#[program]
pub mod chip_fun_sc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64, id: String) -> Result<()> {
        let result = deposit::handler_deposit(ctx, amount, id.clone());

        match result {
            Ok(()) => {
                let current = Clock::get()?;
                emit!(Deposited {
                    id,
                    amount,
                    timestamp: current.unix_timestamp
                });
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, id: String) -> Result<()> {
        let result = withdraw::handler_withdraw(ctx, amount, id.clone());

        match result {
            Ok(()) => {
                let current = Clock::get()?;
                emit!(Withdrawn {
                    id,
                    amount,
                    timestamp: current.unix_timestamp
                });
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn config(
        ctx: Context<Config>,
        new_operator: Option<Pubkey>,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        handle_update(ctx, new_operator, new_authority)
    }
}
