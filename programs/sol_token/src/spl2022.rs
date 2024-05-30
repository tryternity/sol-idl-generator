use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[account]
#[derive(Default)]
pub struct TokenMetadata {
    /// The authority that can sign to update the metadata
    pub update_authority: Pubkey,
    /// The associated mint, used to counter spoofing to be sure that metadata
    /// belongs to a particular mint
    pub mint: Pubkey,
    /// The longer name of the token
    pub name: String,
    /// The shortened symbol for the token
    pub symbol: String,
    /// The URI pointing to richer metadata
    pub uri: String,
    /// Any additional metadata about the token as key-value pairs. The program
    /// must avoid storing the same key twice.
    pub additional_metadata: Vec<(String, String)>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TokenMetadataInstruction {
    Initialize(Initialize),
    UpdateField(UpdateField),
    RemoveKey(RemoveKey),
    UpdateAuthority(UpdateAuthority),
    Emit(Emit),
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct Initialize {
    /// Longer name of the token
    pub name: String,
    /// Shortened symbol of the token
    pub symbol: String,
    /// URI pointing to more metadata (image, video, etc.)
    pub uri: String,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct UpdateField {
    /// Field to update in the metadata
    pub field: Field,
    /// Value to write for the field
    pub value: String,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct RemoveKey {
    /// If the idempotent flag is set to true, then the instruction will not
    /// error if the key does not exist
    pub idempotent: bool,
    /// Key to remove in the additional metadata portion
    pub key: String,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct UpdateAuthority {
    /// New authority for the token metadata, or unset if `None`
    pub new_authority: Pubkey,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct Emit {
    /// Start of range of data to emit
    pub start: Option<u64>,
    /// End of range of data to emit
    pub end: Option<u64>,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub enum Field {
    /// The name field, corresponding to `TokenMetadata.name`
    #[default]
    Name,
    /// The symbol field, corresponding to `TokenMetadata.symbol`
    Symbol,
    /// The uri field, corresponding to `TokenMetadata.uri`
    Uri,
    /// A user field, whose key is given by the associated string
    Key(String),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TokenGroupInstruction {
    InitializeGroup(InitializeGroup),
    UpdateGroupMaxSize(UpdateGroupMaxSize),
    UpdateGroupAuthority(UpdateGroupAuthority),
    InitializeMember,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct InitializeGroup {
    /// Update authority for the group
    pub update_authority: Pubkey,
    /// The maximum number of group members
    pub max_size: [u8; 4],
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct UpdateGroupMaxSize {
    /// New max size for the group
    pub max_size: [u8; 4],
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct UpdateGroupAuthority {
    /// New authority for the group, or unset if `None`
    pub new_authority: Pubkey,
}

#[derive(Default, AnchorDeserialize, AnchorSerialize, Clone, Eq, PartialEq, Debug)]
pub struct MetadataPointer {
    /// Authority that can set the metadata address
    pub authority: Pubkey,
    /// Account address that holds the metadata
    pub metadata_address: Pubkey,
}

