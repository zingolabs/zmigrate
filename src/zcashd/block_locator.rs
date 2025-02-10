use anyhow::Result;

use crate::{ parse, Parse, Parser };

use super::{ ClientVersion, u256 };

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
