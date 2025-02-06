use anyhow::Result;
use crate::{Data, Parseable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transaction {
    pub data: Data,
}

impl Transaction {
    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl Parseable for Transaction {
    fn parse_type() -> &'static str {
        "Transaction"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let data = parser.rest();
        Ok(Self { data })
    }
}
