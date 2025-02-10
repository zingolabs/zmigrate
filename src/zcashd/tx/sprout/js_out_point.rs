use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JSOutPoint {
    pub hash: u256,
    pub js: u64,
    pub n: u8,
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
