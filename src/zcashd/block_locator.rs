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
    fn parse(parser: &mut Parser) -> Result<Self> {
        let version = parse!(parser, "BlockLocator version")?;
        let blocks = parse!(parser, "BlockLocator blocks")?;
        Ok(Self { version, blocks })
    }
}
