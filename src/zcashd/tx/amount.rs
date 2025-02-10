use anyhow::Result;

use crate::{format_zats_as_zec, parse, Parse, Parser};

pub type ZatBalance = Amount;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Amount(pub u64);

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

impl std::fmt::Debug for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Amount({})", format_zats_as_zec(*self))
    }
}

impl Parse for Amount {
    fn parse(p: &mut Parser) -> Result<Self> {
        let amount = parse!(p, "amount")?;
        Ok(Self(amount))
    }
}
