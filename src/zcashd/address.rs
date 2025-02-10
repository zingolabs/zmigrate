use anyhow::Result;
use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub String);

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl Parse for Address {
    fn parse(p: &mut Parser) -> Result<Self> {
        let address = parse!(p, "address")?;
        Ok(Self(address))
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
