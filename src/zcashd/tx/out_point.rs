use anyhow::{Result, Context};

use crate::{u256, Parseable};

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

impl Parseable for OutPoint {
    fn parse_type() -> &'static str {
        "OutPoint"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let txid = u256::parse(parser)
            .context("Parsing out point txid")?;
        let vout = u32::parse(parser)
            .context("Parsing out point vout")?;
        Ok(Self {
            txid,
            vout,
        })
    }
}
