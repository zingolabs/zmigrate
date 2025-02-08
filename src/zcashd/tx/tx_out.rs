use anyhow::{ Result, Context };

use crate::{Parse, Parser};

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
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let value = Amount::parse(parser)
            .context("Parsing transaction output value")?;

        let script_pub_key = Script::parse(parser)
            .context("Parsing transaction output script")?;

        Ok(Self {
            value,
            script_pub_key,
        })
    }
}
