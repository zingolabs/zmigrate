use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JSOutPoint {
    hash: u256,
    js: u64,
    n: u8,
}

impl JSOutPoint {
    pub fn hash(&self) -> &u256 {
        &self.hash
    }

    pub fn js(&self) -> u64 {
        self.js
    }

    pub fn n(&self) -> u8 {
        self.n
    }
}

impl Parse for JSOutPoint {
    fn parse(p: &mut Parser) -> Result<Self> {
        let hash = parse!(p, "out point txid")?;
        let js = parse!(p, "out point vout")?;
        let n = parse!(p, "out point n")?;
        Ok(Self {
            hash,
            js,
            n,
        })
    }
}
