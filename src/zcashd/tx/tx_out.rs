use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{Amount, Script};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TxOut {
    pub value: Amount,
    pub script_pub_key: Script,
}

impl Parse for TxOut {
    fn parse(p: &mut Parser) -> Result<Self> {
        let value = parse!(p, "value")?;
        let script_pub_key = parse!(p, "script_pub_key")?;
        Ok(Self {
            value,
            script_pub_key,
        })
    }
}
