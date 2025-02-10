use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{Proof, RedPallasSignature};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAuthorized {
    pub proof: Proof,
    pub binding_signature: RedPallasSignature,
}

impl Parse for OrchardAuthorized {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            proof: parse!(parser, "proof")?,
            binding_signature: parse!(parser, "binding_signature")?,
        })
    }
}
