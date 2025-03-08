use anyhow::Result;

use crate::{parse, u256, Blob, Parse, Parser};


#[derive(Debug, Clone)]
pub struct ExtendedKeyAttrs {
    pub depth: u8,
    pub parent_fingerprint: Blob<4>,
    pub child_number: u32,
    pub chain_code: u256,
}

impl Parse for ExtendedKeyAttrs {
    fn parse(p: &mut Parser) -> Result<Self> {
        let depth = parse!(p, "depth")?;
        let parent_fingerprint = parse!(p, "parent_fingerprint")?;
        let child_number = parse!(p, "child_number")?;
        let chain_code = parse!(p, "chain_code")?;
        Ok(Self {
            depth,
            parent_fingerprint,
            child_number,
            chain_code,
        })
    }
}
