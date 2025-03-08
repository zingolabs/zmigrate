use anyhow::Result;

use crate::{parse, Parse, Parser, TxId};

pub type SaplingOutPoint = OutPoint;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: TxId,
    pub vout: u32,
}

impl Parse for OutPoint {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self { txid: parse!(p, "txid")?, vout: parse!(p, "vout")? })
    }
}
