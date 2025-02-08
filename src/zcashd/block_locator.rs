use anyhow::{ Result, Context };

use crate::{Parseable, Parser};

use super::{ClientVersion, u256};

/// Vector of block hashes
#[derive(Debug, Clone, PartialEq)]
pub struct BlockLocator {
    version: ClientVersion,
    blocks: Vec<u256>,
}

impl BlockLocator {
    pub fn version(&self) -> &ClientVersion {
        &self.version
    }

    pub fn blocks(&self) -> &[u256] {
        &self.blocks
    }
}

impl Parseable for BlockLocator {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let version = ClientVersion::parse(parser)?;
        let blocks = Vec::parse(parser).context("Parsing BlockLocator")?;
        Ok(Self { version, blocks })
    }
}
