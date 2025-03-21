use std::collections::HashMap;

use crate::u256;

use super::{UnifiedAccountMetadata, UnifiedAddressMetadata};

#[derive(Debug, Clone, PartialEq)]
pub struct UnifiedAccounts {
    address_metadata: HashMap<u256, UnifiedAddressMetadata>,
    full_viewing_keys: HashMap<u256, String>,
    account_metadata: HashMap<u256, UnifiedAccountMetadata>,
}

impl UnifiedAccounts {
    pub fn get_address_metadata(&self, id: u256) -> Option<&UnifiedAddressMetadata> {
        self.address_metadata.get(&id)
    }

    pub fn get_full_viewing_key(&self, id: u256) -> Option<&String> {
        self.full_viewing_keys.get(&id)
    }

    pub fn get_account_metadata(&self, id: u256) -> Option<&UnifiedAccountMetadata> {
        self.account_metadata.get(&id)
    }

    pub fn address_metadata(&self) -> impl Iterator<Item = (&u256, &UnifiedAddressMetadata)> {
        self.address_metadata.iter()
    }

    pub fn full_viewing_keys(&self) -> impl Iterator<Item = (&u256, &String)> {
        self.full_viewing_keys.iter()
    }

    pub fn account_metadata(&self) -> impl Iterator<Item = (&u256, &UnifiedAccountMetadata)> {
        self.account_metadata.iter()
    }
}

impl UnifiedAccounts {
    pub fn new(
        address_metadata: HashMap<u256, UnifiedAddressMetadata>,
        full_viewing_keys: HashMap<u256, String>,
        account_metadata: HashMap<u256, UnifiedAccountMetadata>,
    ) -> Self {
        Self {
            address_metadata,
            full_viewing_keys,
            account_metadata,
        }
    }
}
