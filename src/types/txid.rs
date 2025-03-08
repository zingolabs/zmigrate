use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::u256;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct TxId(pub u256);

impl Parse for TxId {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(p, "txid")?))
    }
}
impl std::fmt::Debug for TxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TxId({})", self.0)
    }
}

impl std::fmt::Display for TxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
