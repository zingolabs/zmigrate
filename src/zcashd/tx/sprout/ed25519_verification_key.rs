use anyhow::Result;

use crate::{ parse, Blob32, Parse, Parser };

#[derive(Debug, Clone, PartialEq)]
pub struct Ed25519VerificationKey(pub Blob32);

impl Parse for Ed25519VerificationKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "Ed25519VerificationKey")?))
    }
}
