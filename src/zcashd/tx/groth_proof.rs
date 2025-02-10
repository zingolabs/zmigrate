use anyhow::Result;

use crate::{ parse, Blob, Parse, Parser };

pub const GROTH_PROOF_SIZE: usize = 48 + 96 + 48;

#[derive(Debug, Clone, PartialEq)]
pub struct GrothProof(pub Blob<GROTH_PROOF_SIZE>);

impl Parse for GrothProof {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "GrothProof")?))
    }
}
