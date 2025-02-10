use anyhow::Result;

use crate::{ parse, Blob64, Parse, Parser };

#[derive(Debug, Clone, PartialEq)]
pub struct Ed25519Signature(pub Blob64);

impl Parse for Ed25519Signature {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "Ed25519Signature")?))
    }
}
