use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Mint {
    // COption
    pub mint_authority: Option<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    // COption
    pub freeze_authority: Option<Pubkey>,
}

#[account]
#[derive(Default)]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub delegate: Option<Pubkey>,
    pub state: AccountState,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AccountState {
    Uninitialized,

    Initialized,

    Frozen,
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState::Uninitialized
    }
}