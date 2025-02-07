use anyhow::{ Result, Context };

use crate::Parseable;

use super::{parse_array, ClientVersion, U256};

/// Vector of block hashes
#[derive(Debug, Clone, PartialEq)]
pub struct BlockLocator {
    version: ClientVersion,
    blocks: Vec<U256>,
}

impl BlockLocator {
    pub fn version(&self) -> &ClientVersion {
        &self.version
    }

    pub fn blocks(&self) -> &[U256] {
        &self.blocks
    }
}

impl Parseable for BlockLocator {
    fn parse_type() -> &'static str {
        "BlockLocator"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let version = ClientVersion::parse(parser)?;
        let blocks = parse_array(parser).context("Parsing BlockLocator")?;
        Ok(Self { version, blocks })
    }
}
