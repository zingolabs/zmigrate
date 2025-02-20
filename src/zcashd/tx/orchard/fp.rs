use anyhow::Result;

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct Fp(pub [u64; 4]);

impl Parse for Fp {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(parser, "Fp")?))
    }
}
