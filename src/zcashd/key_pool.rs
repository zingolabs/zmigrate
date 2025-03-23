use anyhow::Result;

use crate::{Parse, Parser, SecondsSinceEpoch, parse};

use super::{ClientVersion, PubKey};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyPoolEntry {
    version: ClientVersion,
    timestamp: SecondsSinceEpoch,
    key: PubKey,
}

impl KeyPoolEntry {
    pub fn version(&self) -> ClientVersion {
        self.version
    }

    pub fn timestamp(&self) -> SecondsSinceEpoch {
        self.timestamp
    }

    pub fn key(&self) -> &PubKey {
        &self.key
    }
}

impl Parse for KeyPoolEntry {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            version: parse!(p, "version")?,
            timestamp: parse!(p, "timestamp")?,
            key: parse!(p, "key")?,
        })
    }
}
