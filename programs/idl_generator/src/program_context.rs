use anchor_lang::prelude::*;

use crate::metadata::Metadata;
use crate::mint::Mint;

#[derive(Accounts)]
pub struct ProgramContext<'info> {
    #[account(mut)]
    pub meta_acc: Box<Account<'info, Metadata>>,

    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,
}

impl<'info> ProgramContext<'info> {}
