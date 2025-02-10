use anyhow::Result;

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct Proof(pub Vec<u8>);

impl Parse for Proof {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(parser, "Proof")?))
    }
}
