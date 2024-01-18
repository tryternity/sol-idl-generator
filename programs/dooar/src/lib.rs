use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

//https://github.com/lhendre/metaplexDemo
declare_id!("5oxtLp6fKgYuo6H6BVaiENk6z3adrkzgvEjhSWUe2MF5");

#[program]
pub mod dooar {
    use super::*;

    pub fn swap(_ctx: Context<SwapContext>, _amount_in: u64, _minimum_amount_out: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapContext<'info> {
    /// CHECK:
    pub token_swap: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    pub user_transfer_authority: AccountInfo<'info>,
    /// CHECK:
    pub user_source: AccountInfo<'info>,
    /// CHECK:
    pub pool_source: AccountInfo<'info>,
    /// CHECK:
    pub pool_destination: AccountInfo<'info>,
    /// CHECK:
    pub user_destination: AccountInfo<'info>,
    /// CHECK:
    pub pool_mint: AccountInfo<'info>,
    /// CHECK:
    pub pool_fee_account: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

impl<'info> SwapContext<'info> {}

