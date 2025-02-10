use anyhow::Result;

use crate::Blob64;

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardSignature(Blob64);

impl Parse for OrchardSignature {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(parser, "OrchardSignature")?))
    }
}
