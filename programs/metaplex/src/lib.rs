use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::{
    program_context::*,
};
use crate::metadata::*;

mod metadata;
mod data_struct;

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
}

impl<'info> ProgramContext<'info> {}