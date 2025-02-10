use anyhow::Result;

use crate::Blob32;

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct RedPallasVerificationKey {
    point: Blob32,
    bytes: Blob32,
}

impl Parse for RedPallasVerificationKey {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            point: parse!(parser, "RedPallasVerificationKey.point")?,
            bytes: parse!(parser, "RedPallasVerificationKey.bytes")?,
        })
    }
}
