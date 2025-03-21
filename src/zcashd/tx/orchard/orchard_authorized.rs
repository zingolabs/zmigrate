use anyhow::Result;

use crate::{Data, Parse, Parser, parse};

use super::RedPallasSignature;

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAuthorized {
    pub proof: Data,
    pub binding_signature: RedPallasSignature,
}

impl Parse for OrchardAuthorized {
    fn parse(parser: &mut Parser) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            proof: parse!(parser, "proof")?,
            binding_signature: parse!(parser, "binding_signature")?,
        })
    }
}
