use anyhow::Result;

use crate::{Blob32, Parse, Parser, parse};

#[derive(Debug, Clone, PartialEq)]
pub struct RedPallasSignature {
    r_bytes: Blob32,
    s_bytes: Blob32,
}

impl Parse for RedPallasSignature {
    fn parse(parser: &mut Parser) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            r_bytes: parse!(parser, "r_bytes")?,
            s_bytes: parse!(parser, "s_bytes")?,
        })
    }
}
