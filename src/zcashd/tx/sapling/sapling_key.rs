use anyhow::Result;

use crate::{sapling::SaplingExtendedSpendingKey, SaplingIncomingViewingKey};

use super::super::super::KeyMetadata;

#[derive(Debug, Clone, PartialEq)]
pub struct SaplingKey {
    pub ivk: SaplingIncomingViewingKey,
    pub key: SaplingExtendedSpendingKey,
    pub metadata: KeyMetadata,
}

impl SaplingKey {
    pub fn new(
        ivk: SaplingIncomingViewingKey,
        key: SaplingExtendedSpendingKey,
        metadata: KeyMetadata,
    ) -> Result<Self> {
        Ok(Self { ivk, key, metadata })
    }
}
