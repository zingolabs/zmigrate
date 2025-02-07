use anyhow::Result;
use crate::{ Data, Parseable };

use super::TxVersion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WalletTx {
    pub version: TxVersion,
    pub rest: Data,
}

impl WalletTx {}

impl Parseable for WalletTx {
    fn parse_type() -> &'static str {
        "Transaction"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let version = TxVersion::parse(parser)?;

        let rest = parser.rest();
        Ok(Self {
            version,
            rest,
        })
    }
}
