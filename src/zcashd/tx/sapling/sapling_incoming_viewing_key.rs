use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingIncomingViewingKey(pub u256);

impl Parse for SaplingIncomingViewingKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "SaplingIncomingViewingKey")?))
    }
}
