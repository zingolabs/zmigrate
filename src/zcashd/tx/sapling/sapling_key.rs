use anyhow::Result;

use super::{super::super::KeyMetadata, SaplingExtendedSpendingKey, SaplingIncomingViewingKey};

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
