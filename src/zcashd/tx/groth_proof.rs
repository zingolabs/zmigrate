use anyhow::{ Result, Context };

use crate::{ Blob, Parse, Parser };

pub const GROTH_PROOF_SIZE: usize = 48 + 96 + 48;

#[derive(Clone, PartialEq)]
pub struct GrothProof(Blob<GROTH_PROOF_SIZE>);

impl AsRef<Blob<GROTH_PROOF_SIZE>> for GrothProof {
    fn as_ref(&self) -> &Blob<GROTH_PROOF_SIZE> {
        &self.0
    }
}

impl AsRef<[u8]> for GrothProof {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Parse for GrothProof {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let blob = Parse::parse(parser).context("Parsing GrothProof")?;
        Ok(Self(blob))
    }
}

impl std::fmt::Debug for GrothProof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GrothProof({})", hex::encode(self))
    }
}
