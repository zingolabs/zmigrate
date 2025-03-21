use anyhow::Result;

use crate::{Blob, Parse, Parser, parse, u256};

use super::ReceiverType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnifiedAddressMetadata {
    key_id: u256,
    diversifier_index: Blob<11>,
    receiver_types: Vec<ReceiverType>,
}

impl UnifiedAddressMetadata {
    pub fn key_id(&self) -> u256 {
        self.key_id
    }

    pub fn diversifier_index(&self) -> &Blob<11> {
        &self.diversifier_index
    }

    pub fn receiver_types(&self) -> &[ReceiverType] {
        &self.receiver_types
    }
}

impl Parse for UnifiedAddressMetadata {
    fn parse(p: &mut Parser) -> Result<Self> {
        let key_id = parse!(p, "key_id")?;
        let diversifier_index = parse!(p, "diversifier_index")?;
        let receiver_types = parse!(p, "receiver_types")?;
        Ok(Self { key_id, diversifier_index, receiver_types })
    }
}
