use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::mint::*;
use crate::spl2022::*;

mod mint;
mod spl2022;

//https://github.com/lhendre/metaplexDemo
declare_id!("5oxtLp6fKgYuo6H6BVaiENk6z3adrkzgvEjhSWUe2MF5");

#[program]
pub mod spl_token {
    use super::*;

    pub fn hello_word(_ctx: Context<ProgramContext>,
                      _p1: TokenMetadataInstruction,
                      _p2: TokenGroupInstruction,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProgramContext<'info> {
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub metadata: Box<Account<'info, TokenMetadata>>,
}

impl<'info> ProgramContext<'info> {}