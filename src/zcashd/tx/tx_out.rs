use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{Amount, Script};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TxOut {
    value: Amount,
    script_pub_key: Script,
}

impl TxOut {
    pub fn value(&self) -> &Amount {
        &self.value
    }

    pub fn script_pub_key(&self) -> &Script {
        &self.script_pub_key
    }
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
