use std::collections::HashMap;

use anchor_lang::prelude::*;

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
// pub struct UpdateMetadataAccountArgs {
//     pub data: Option<Data>,
//     #[cfg_attr(
//     feature = "serde-feature",
//     serde(with = "As::<Option<DisplayFromStr>>")
//     )]
//     pub update_authority: Option<Pubkey>,
//     pub primary_sale_happened: Option<bool>,
// }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct UpdateMetadataAccountArgsV2 {
    pub data: Option<DataV2>,
    #[cfg_attr(
    feature = "serde-feature",
    serde(with = "As::<Option<DisplayFromStr>>")
    )]
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
// pub struct CreateMetadataAccountArgs {
//     /// Note that unique metadatas are disabled for now.
//     pub data: Data,
//     /// Whether you want your metadata to be updateable in the future.
//     pub is_mutable: bool,
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
// pub struct CreateMetadataAccountArgsV2 {
//     /// Note that unique metadatas are disabled for now.
//     pub data: DataV2,
//     /// Whether you want your metadata to be updateable in the future.
//     pub is_mutable: bool,
// }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct CreateMetadataAccountArgsV3 {
    /// Note that unique metadatas are disabled for now.
    pub data: DataV2,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
    /// If this is a collection parent NFT.
    pub collection_details: Option<CollectionDetails>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct CreateMasterEditionArgs {
    /// If set, means that no more than this number of editions can ever be minted. This is immutable.
    pub max_supply: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct MintNewEditionFromMasterEditionViaTokenArgs {
    pub edition: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ApproveUseAuthorityArgs {
    pub number_of_uses: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct UtilizeArgs {
    pub number_of_uses: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, PartialEq, Eq, Debug)]
pub struct Data {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct DataV2 {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct Creator {
    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum CollectionDetails {
    V1 { size: u64 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: u8,
    //1
    pub remaining: u64,
    //8
    pub total: u64,            //8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum TokenStandard {
    NonFungible,
    // This is a master edition
    FungibleAsset,
    // A token with metadata that can also have attrributes
    Fungible,
    // A token with simple metadata
    NonFungibleEdition,
    // This is a limited edition
    ProgrammableNonFungible, // NonFungible with programmable configuration
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct SetCollectionSizeArgs {
    pub size: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct TransferOutOfEscrowArgs {
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum BurnArgs {
    V1 {
        /// The amount of the token to burn
        amount: u64,
    },
}


#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreateArgs {
    V1 {
        asset_data: AssetData,
        decimals: Option<u8>,
        print_supply: Option<PrintSupply>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AssetData {
    /// The name of the asset.
    pub name: String,
    /// The symbol for the asset.
    pub symbol: String,
    /// URI pointing to JSON representing the asset.
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000).
    pub seller_fee_basis_points: u16,
    /// Array of creators.
    pub creators: Option<Vec<Creator>>,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable (default is not).
    pub is_mutable: bool,
    /// Type of the token.
    pub token_standard: TokenStandard,
    /// Collection information.
    pub collection: Option<Collection>,
    /// Uses information.
    pub uses: Option<Uses>,
    /// Collection item details.
    pub collection_details: Option<CollectionDetails>,
    /// Programmable rule set for the asset.
    #[cfg_attr(
    feature = "serde-feature",
    serde(
    deserialize_with = "deser_option_pubkey",
    serialize_with = "ser_option_pubkey"
    )
    )]
    pub rule_set: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
/// Represents the print supply of a non-fungible asset.
pub enum PrintSupply {
    /// The asset does not have any prints.
    Zero,
    /// The asset has a limited amount of prints.
    Limited(u64),
    /// The asset has an unlimited amount of prints.
    Unlimited,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum MintArgs {
    V1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthorizationData {
    pub payload: Payload,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone, Default, )]
pub struct Payload {
    map: HashMap<String, PayloadType>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum PayloadType {
    /// A plain `Pubkey`.
    Pubkey(Pubkey),
    /// PDA derivation seeds.
    Seeds(SeedsVec),
    /// A merkle proof.
    MerkleProof(ProofInfo),
    /// A plain `u64` used for `Amount`.
    Number(u64),
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct SeedsVec {
    /// The vector of derivation seeds.
    pub seeds: Vec<Vec<u8>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct ProofInfo {
    /// The merkle proof.
    pub proof: Vec<[u8; 32]>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum DelegateArgs {
    CollectionV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    SaleV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    TransferV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    DataV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    UtilityV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    StakingV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    StandardV1 {
        amount: u64,
    },
    LockedTransferV1 {
        amount: u64,
        /// locked destination pubkey
        locked_address: Pubkey,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    ProgrammableConfigV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AuthorityItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    DataItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    CollectionItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    ProgrammableConfigItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum RevokeArgs {
    CollectionV1,
    SaleV1,
    TransferV1,
    DataV1,
    UtilityV1,
    StakingV1,
    StandardV1,
    LockedTransferV1,
    ProgrammableConfigV1,
    MigrationV1,
    AuthorityItemV1,
    DataItemV1,
    CollectionItemV1,
    ProgrammableConfigItemV1,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum LockArgs {
    V1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UnlockArgs {
    V1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum MigrateArgs {
    V1 {
        migration_type: MigrationType,
        rule_set: Option<Pubkey>,
    },
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq)]
pub enum MigrationType {
    CollectionV1,
    ProgrammableV1,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum TransferArgs {
    V1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UpdateArgs {
    V1 {
        /// The new update authority.
        new_update_authority: Option<Pubkey>,
        /// The metadata details.
        data: Option<Data>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Collection information.
        collection: CollectionToggle,
        /// Additional details of the collection.
        collection_details: CollectionDetailsToggle,
        /// Uses information.
        uses: UsesToggle,
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsUpdateAuthorityV2 {
        /// The new update authority.
        new_update_authority: Option<Pubkey>,
        /// The metadata details.
        data: Option<Data>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Collection information.
        collection: CollectionToggle,
        /// Additional details of the collection.
        collection_details: CollectionDetailsToggle,
        /// Uses information.
        uses: UsesToggle,
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Token standard.
        token_standard: Option<TokenStandard>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsAuthorityItemDelegateV2 {
        /// The new update authority.
        new_update_authority: Option<Pubkey>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Token standard.
        token_standard: Option<TokenStandard>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsCollectionDelegateV2 {
        /// Collection information.
        collection: CollectionToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsDataDelegateV2 {
        /// The metadata details.
        data: Option<Data>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsProgrammableConfigDelegateV2 {
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsDataItemDelegateV2 {
        /// The metadata details.
        data: Option<Data>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsCollectionItemDelegateV2 {
        /// Collection information.
        collection: CollectionToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsProgrammableConfigItemDelegateV2 {
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}


#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum CollectionToggle {
    #[default]
    None,
    Clear,
    Set(Collection),
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum CollectionDetailsToggle {
    #[default]
    None,
    Clear,
    Set(CollectionDetails),
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum UsesToggle {
    #[default]
    None,
    Clear,
    Set(Uses),
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum RuleSetToggle {
    #[default]
    None,
    Clear,
    Set(Pubkey),
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UseArgs {
    V1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum VerificationArgs {
    CreatorV1,
    CollectionV1,
}

#[cfg(test)]
pub mod tests {
    use anchor_lang::AnchorSerialize;

    use crate::data_struct::Payload;

    #[test]
    fn payload() {
        let p = Payload { map: Default::default() };
        let s = p.try_to_vec();
        print!("{:?}", s.unwrap());
    }
}