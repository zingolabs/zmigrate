use anyhow::Result;

use crate::{Parse, Parser, parse, u256};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JSOutPoint {
    pub hash: u256,
    pub js: u64,
    pub n: u8,
}

impl Parse for JSOutPoint {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            hash: parse!(p, "hash")?,
            js: parse!(p, "js")?,
            n: parse!(p, "n")?,
        })
    }
}
