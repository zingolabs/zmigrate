use anyhow::Result;
use crate::{parse, Parse, Parser, SecondsSinceEpoch};

use super::{ClientVersion, PubKey};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyPoolEntry {
    pub version: ClientVersion,
    pub timestamp: SecondsSinceEpoch,
    pub key: PubKey,
}

impl KeyPoolEntry {
    pub fn version(&self) -> &ClientVersion {
        &self.version
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
        let version = parse!(p, "version")?;
        let timestamp = parse!(p, "timestamp")?;
        let key = parse!(p, "key")?;
        Ok(Self { version, timestamp, key })
    }
}
