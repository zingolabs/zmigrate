use anyhow::{Result, Context};

use crate::{Parse, Parser};

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

impl Parse for TxIn {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let prevout = Parse::parse(parser)
            .context("txin prevout")?;
        let script_sig = Parse::parse(parser)
            .context("txin script_sig")?;
        let sequence = Parse::parse(parser)
            .context("txin sequence")?;
        Ok(Self {
            prevout,
            script_sig,
            sequence,
        })
    }
}
