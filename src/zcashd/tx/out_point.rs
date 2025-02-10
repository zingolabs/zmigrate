use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

pub type SaplingOutPoint = OutPoint;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: u256,
    pub vout: u32,
}

impl Parse for OutPoint {
    fn parse(p: &mut Parser) -> Result<Self> {
        let txid = parse!(p, "out point txid")?;
        let vout = parse!(p, "out point vout")?;
        Ok(Self {
            txid,
            vout,
        })
    }
}
