use anyhow::Result;

use crate::{format_zec, parse, Parse, Parser};

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Amount(u64);

impl Amount {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<Amount> for u64 {
    fn from(amount: Amount) -> Self {
        amount.0
    }
}

impl From<u64> for Amount {
    fn from(amount: u64) -> Self {
        Self(amount)
    }
}

impl From<&Amount> for u64 {
    fn from(amount: &Amount) -> Self {
        amount.0
    }
}

impl From<&u64> for Amount {
    fn from(amount: &u64) -> Self {
        Self(*amount)
    }
}

impl std::fmt::Debug for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Amount({})", format_zec(self))
    }
}

impl Parse for Amount {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        let amount = parse!(p, "amount")?;
        Ok(Self(amount))
    }
}
