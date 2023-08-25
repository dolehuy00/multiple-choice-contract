use near_sdk::collections::{LazyOption, UnorderedSet, UnorderedMap};
use near_sdk::{near_bindgen, CryptoHash, PanicOnDefault};
use near_sdk::{AccountId, collections::LookupMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{Base64VecU8, U128};

mod  mutilple_choice;
use mutilple_choice::{Question};

pub type TokenId = String;


pub use crate::metadata::*;

mod metadata;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {
    pub owner_id: AccountId,

    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, // Lưu danh sách token mà user sở hữu

    pub tokens_by_id: LookupMap<TokenId, Token>, // Mapping token id với các data mở rộng của token

    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, // Mapping token id với token metadata

    pub metadata: LazyOption<NFTContractMetadata>,
    
    pub questions: UnorderedMap<String, Question>
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    TokenPerOwnerKey,
    ContractMetadataKey,
    TokenByIdKey,
    TokenMetadataByIdKey,
    TokenPerOwnerInnerKey {
        account_id_hash: CryptoHash
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, token_metadata: NFTContractMetadata) -> Self {
        Self {
            owner_id,
            metadata: LazyOption::new(
                StorageKey::ContractMetadataKey.try_to_vec().unwrap(),
                Some(&token_metadata)
            ),
            tokens_per_owner: LookupMap::new(StorageKey::TokenPerOwnerKey.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokenByIdKey.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadataByIdKey.try_to_vec().unwrap()),
            questions: UnorderedMap::new(b"questions".to_vec()),
        }
    }

    #[init]
    pub fn new_default_metadata(owner_id: AccountId) -> Self {
        Self::new(
            owner_id, 
        NFTContractMetadata {
            spec: "nft-mutilple-choice-1.0.0".to_string(),
            name: "Mutiple-choice".to_string(),
            symbol: "VNFT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None
        })
    }
}
