use anyhow::Result;

use crate::{Parse, Parser, parse, u256};

use super::ClientVersion;

/// Vector of block hashes
#[derive(Debug, Clone, PartialEq)]
pub struct BlockLocator {
    pub version: ClientVersion,
    pub blocks: Vec<u256>,
}

impl Parse for BlockLocator {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            version: parse!(p, "version")?,
            blocks: parse!(p, "blocks")?,
        })
    }
}
