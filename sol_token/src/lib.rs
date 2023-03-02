use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::{
    program_context::*,
};

mod mint;

//https://github.com/lhendre/metaplexDemo
declare_id!("5oxtLp6fKgYuo6H6BVaiENk6z3adrkzgvEjhSWUe2MF5");

#[program]
pub mod idl_generator {
    use super::*;

    pub fn hello_word(_ctx: Context<ProgramContext>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProgramContext<'info> {
    #[account(mut)]
    pub meta_acc: Box<Account<'info, Metadata>>,

    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,
}

impl<'info> ProgramContext<'info> {}