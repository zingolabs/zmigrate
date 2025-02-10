use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{OutPoint, Script};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TxIn {
    pub prevout: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl Parse for TxIn {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            prevout: parse!(p, "txin prevout")?,
            script_sig: parse!(p, "txin script_sig")?,
            sequence: parse!(p, "txin sequence")?,
        })
    }
}
