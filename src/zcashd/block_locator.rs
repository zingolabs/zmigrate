use anyhow::Result;

use crate::{ parse, Parse, Parser };

use super::{ ClientVersion, u256 };

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

impl Parse for BlockLocator {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version = parse!(p, "BlockLocator version")?;
        let blocks = parse!(p, "BlockLocator blocks")?;
        Ok(Self { version, blocks })
    }
}
