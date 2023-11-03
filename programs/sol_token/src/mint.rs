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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TransferFeeInstruction {
    /// Initialize the transfer fee on a new mint.
    ///
    /// Fails if the mint has already been initialized, so must be called before
    /// `InitializeMint`.
    ///
    /// The mint must have exactly enough space allocated for the base mint (82
    /// bytes), plus 83 bytes of padding, 1 byte reserved for the account type,
    /// then space required for this extension, plus any others.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The mint to initialize.
    InitializeTransferFeeConfig {
        /// Pubkey that may update the fees
        #[cfg_attr(feature = "serde-traits", serde(with = "coption_fromstr"))]
        transfer_fee_config_authority: Option<Pubkey>,
        /// Withdraw instructions must be signed by this key
        #[cfg_attr(feature = "serde-traits", serde(with = "coption_fromstr"))]
        withdraw_withheld_authority: Option<Pubkey>,
        /// Amount of transfer collected as fees, expressed as basis points of the
        /// transfer amount
        transfer_fee_basis_points: u16,
        /// Maximum fee assessed on transfers
        maximum_fee: u64,
    },
    /// Transfer, providing expected mint information and fees
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The source account. Must include the `TransferFeeAmount` extension.
    ///   1. `[]` The token mint. Must include the `TransferFeeConfig` extension.
    ///   2. `[writable]` The destination account. Must include the `TransferFeeAmount` extension.
    ///   3. `[signer]` The source account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[]` The token mint.
    ///   2. `[writable]` The destination account.
    ///   3. `[]` The source account's multisignature owner/delegate.
    ///   4. ..4+M `[signer]` M signer accounts.
    TransferCheckedWithFee {
        /// The amount of tokens to transfer.
        amount: u64,
        /// Expected number of base 10 digits to the right of the decimal place.
        decimals: u8,
        /// Expected fee assessed on this transfer, calculated off-chain based on
        /// the transfer_fee_basis_points and maximum_fee of the mint.
        fee: u64,
    },
    /// Transfer all withheld tokens in the mint to an account. Signed by the mint's
    /// withdraw withheld tokens authority.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The token mint. Must include the `TransferFeeConfig` extension.
    ///   1. `[writable]` The fee receiver account. Must include the `TransferFeeAmount` extension
    ///      associated with the provided mint.
    ///   2. `[signer]` The mint's `withdraw_withheld_authority`.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The token mint.
    ///   1. `[writable]` The destination account.
    ///   2. `[]` The mint's multisig `withdraw_withheld_authority`.
    ///   3. ..3+M `[signer]` M signer accounts.
    WithdrawWithheldTokensFromMint,
    /// Transfer all withheld tokens to an account. Signed by the mint's
    /// withdraw withheld tokens authority.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[]` The token mint. Must include the `TransferFeeConfig` extension.
    ///   1. `[writable]` The fee receiver account. Must include the `TransferFeeAmount`
    ///      extension and be associated with the provided mint.
    ///   2. `[signer]` The mint's `withdraw_withheld_authority`.
    ///   3. ..3+N `[writable]` The source accounts to withdraw from.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[]` The token mint.
    ///   1. `[writable]` The destination account.
    ///   2. `[]` The mint's multisig `withdraw_withheld_authority`.
    ///   3. ..3+M `[signer]` M signer accounts.
    ///   3+M+1. ..3+M+N `[writable]` The source accounts to withdraw from.
    WithdrawWithheldTokensFromAccounts {
        /// Number of token accounts harvested
        num_token_accounts: u8,
    },
    /// Permissionless instruction to transfer all withheld tokens to the mint.
    ///
    /// Succeeds for frozen accounts.
    ///
    /// Accounts provided should include the `TransferFeeAmount` extension. If not,
    /// the account is skipped.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The mint.
    ///   1. ..1+N `[writable]` The source accounts to harvest from.
    HarvestWithheldTokensToMint,
    /// Set transfer fee. Only supported for mints that include the `TransferFeeConfig` extension.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single authority
    ///   0. `[writable]` The mint.
    ///   1. `[signer]` The mint's fee account owner.
    ///
    ///   * Multisignature authority
    ///   0. `[writable]` The mint.
    ///   1. `[]` The mint's multisignature fee account owner.
    ///   2. ..2+M `[signer]` M signer accounts.
    SetTransferFee {
        /// Amount of transfer collected as fees, expressed as basis points of the
        /// transfer amount
        transfer_fee_basis_points: u16,
        /// Maximum fee assessed on transfers
        maximum_fee: u64,
    },
}

impl Default for TransferFeeInstruction {
    fn default() -> Self {
        todo!()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ExtensionType {
    /// Used as padding if the account size would otherwise be 355, same as a multisig
    Uninitialized,
    /// Includes transfer fee rate info and accompanying authorities to withdraw and set the fee
    TransferFeeConfig,
    /// Includes withheld transfer fees
    TransferFeeAmount,
    /// Includes an optional mint close authority
    MintCloseAuthority,
    /// Auditor configuration for confidential transfers
    ConfidentialTransferMint,
    /// State for confidential transfers
    ConfidentialTransferAccount,
    /// Specifies the default Account::state for new Accounts
    DefaultAccountState,
    /// Indicates that the Account owner authority cannot be changed
    ImmutableOwner,
    /// Require inbound transfers to have memo
    MemoTransfer,
    /// Indicates that the tokens from this mint can't be transfered
    NonTransferable,
    /// Tokens accrue interest over time,
    InterestBearingConfig,
    /// Locks privileged token operations from happening via CPI
    CpiGuard,
    /// Includes an optional permanent delegate
    PermanentDelegate,
    /// Indicates that the tokens in this account belong to a non-transferable mint
    NonTransferableAccount,
    /// Mint requires a CPI to a program implementing the "transfer hook" interface
    TransferHook,
    /// Indicates that the tokens in this account belong to a mint with a transfer hook
    TransferHookAccount,
    /// Includes encrypted withheld fees and the encryption public that they are encrypted under
    ConfidentialTransferFeeConfig,
    /// Includes confidential withheld transfer fees
    ConfidentialTransferFeeAmount,
    /// Mint contains a pointer to another account (or the same account) that holds metadata
    MetadataPointer,
    /// Mint contains token-metadata
    TokenMetadata,
    /// Test variable-length mint extension
    /// Mint contains a pointer to another account (or the same account) that holds group
    /// configurations
    GroupPointer,
    #[cfg(test)]
    VariableLenMintTest = (u16::MAX - 2) as isize,
    /// Padding extension used to make an account exactly Multisig::LEN, used for testing
    #[cfg(test)]
    AccountPaddingTest,
    /// Padding extension used to make a mint exactly Multisig::LEN, used for testing
    #[cfg(test)]
    MintPaddingTest,
}

impl Default for ExtensionType {
    fn default() -> Self {
        ExtensionType::Uninitialized
    }
}