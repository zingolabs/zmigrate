use anyhow::{Result, Context};

use crate::{Blob32, Parseable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: Blob32,
    pub vout: u32,
}

impl OutPoint {
    pub fn txid(&self) -> &Blob32 {
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
        let txid = Blob32::parse(parser)
            .context("Parsing out point txid")?;
        let vout = u32::parse(parser)
            .context("Parsing out point vout")?;
        Ok(Self {
            txid,
            vout,
        })
    }
}
