use anyhow::{Result, Context};

use crate::{Parseable, Parser};

use super::{OutPoint, Script};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TxIn {
    pub prevout: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TxIn {
    pub fn prevout(&self) -> &OutPoint {
        &self.prevout
    }

    pub fn script_sig(&self) -> &Script {
        &self.script_sig
    }

    pub fn sequence(&self) -> u32 {
        self.sequence
    }
}

impl Parseable for TxIn {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let prevout = OutPoint::parse(parser)
            .context("Parsing txin prevout")?;
        let script_sig = Script::parse(parser)
            .context("Parsing txin script_sig")?;
        let sequence = u32::parse(parser)
            .context("Parsing txin sequence")?;
        Ok(Self {
            prevout,
            script_sig,
            sequence,
        })
    }
}
