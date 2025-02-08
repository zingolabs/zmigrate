use anyhow::{Result, Context};

use crate::{u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: u256,
    pub vout: u32,
}

impl OutPoint {
    pub fn txid(&self) -> &u256 {
        &self.txid
    }

    pub fn vout(&self) -> u32 {
        self.vout
    }
}

impl Parse for OutPoint {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let txid = Parse::parse(parser)
            .context("out point txid")?;
        let vout = Parse::parse(parser)
            .context("out point vout")?;
        Ok(Self {
            txid,
            vout,
        })
    }
}
