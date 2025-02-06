use anyhow::{ Result, Context };

use crate::{ Blob32, Parseable };

use super::ClientVersion;

/// Vector of block hashes
#[derive(Debug, Clone, PartialEq)]
pub struct BlockLocator {
    version: ClientVersion,
    blocks: Vec<Blob32>,
}

impl BlockLocator {
    pub fn version(&self) -> &ClientVersion {
        &self.version
    }
    
    pub fn blocks(&self) -> &[Blob32] {
        &self.blocks
    }
}

impl Parseable for BlockLocator {
    fn parse_type() -> &'static str {
        "BlockLocator"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let version = ClientVersion::parse(parser)?;
        let blocks = parser.parse_array().context("Parsing BlockLocator")?;
        Ok(Self { version, blocks })
    }
}
