use anyhow::Result;

use crate::{parse, Parse, Parser};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Address(pub String);

impl Parse for Address {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "address")?))
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Address({})", self.0)
    }
}
