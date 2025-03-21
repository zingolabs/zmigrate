use anyhow::Result;

use crate::{Parse, Parser, SecondsSinceEpoch, parse};

use super::{ClientVersion, PubKey};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyPoolEntry {
    pub version: ClientVersion,
    pub timestamp: SecondsSinceEpoch,
    pub key: PubKey,
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
