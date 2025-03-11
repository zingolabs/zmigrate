use anyhow::Result;
use crate::{Parse, Parser};

pub type TxId = zcash_primitives::transaction::TxId;

impl Parse for TxId {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(TxId::read(p)?)
    }
}
