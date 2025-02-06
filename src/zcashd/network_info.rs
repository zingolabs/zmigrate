use anyhow::Result;
use crate::Parseable;

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfo {
    zcash: String,
    identifier: String,
}

impl NetworkInfo {
    pub fn zcash(&self) -> &str {
        &self.zcash
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl Parseable for NetworkInfo {
    fn parse_type() -> &'static str {
        "NetworkInfo"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let (zcash, identifier) = parser.parse_pair()?;
        Ok(Self { zcash, identifier })
    }
}
