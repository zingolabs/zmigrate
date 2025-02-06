use anyhow::Result;
use crate::Parseable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(String);

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl Parseable for Address {
    fn parse_type() -> &'static str {
        "Address"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let address = parser.parse_utf8()?;
        Ok(Self(address))
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
