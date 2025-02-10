use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{Proof, RedPallasSignature};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAuthorized {
    proof: Proof,
    binding_signature: RedPallasSignature,
}

impl Parse for OrchardAuthorized {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            proof: parse!(parser, "OrchardAuthorized.proof")?,
            binding_signature: parse!(parser, "OrchardAuthorized.binding_signature")?,
        })
    }
}
