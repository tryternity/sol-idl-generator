use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::{
    program_context::*,
};

mod my_struct;
mod program_context;
mod metadata;
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
