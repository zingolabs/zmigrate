use super::{u252, KeyMetadata};

#[derive(Debug, Clone, PartialEq)]
pub struct SproutSpendingKey {
    pub key: u252,
    pub metadata: KeyMetadata,
}

impl SproutSpendingKey {
    pub fn new(key: u252, metadata: KeyMetadata) -> Self {
        Self { key, metadata }
    }
}
