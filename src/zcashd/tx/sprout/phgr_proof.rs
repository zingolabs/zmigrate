use anyhow::Result;

use crate::{Blob, Parse, Parser, parse};

// Initial bool || Fq
pub type CompressedG1 = Blob<33>;

#[derive(Debug, Clone, PartialEq)]
pub struct PHGRProof {
    g_a: CompressedG1,
    g_a_prime: CompressedG1,
    g_b: CompressedG1,
    g_b_prime: CompressedG1,
    g_c: CompressedG1,
    g_c_prime: CompressedG1,
    g_k: CompressedG1,
    g_h: CompressedG1,
}

impl PHGRProof {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(8 * 33);
        result.extend_from_slice(&self.g_a.0);
        result.extend_from_slice(&self.g_a_prime.0);
        result.extend_from_slice(&self.g_b.0);
        result.extend_from_slice(&self.g_b_prime.0);
        result.extend_from_slice(&self.g_c.0);
        result.extend_from_slice(&self.g_c_prime.0);
        result.extend_from_slice(&self.g_k.0);
        result.extend_from_slice(&self.g_h.0);
        result
    }

    pub fn g_a(&self) -> &CompressedG1 {
        &self.g_a
    }

    pub fn g_a_prime(&self) -> &CompressedG1 {
        &self.g_a_prime
    }

    pub fn g_b(&self) -> &CompressedG1 {
        &self.g_b
    }

    pub fn g_b_prime(&self) -> &CompressedG1 {
        &self.g_b_prime
    }

    pub fn g_c(&self) -> &CompressedG1 {
        &self.g_c
    }

    pub fn g_c_prime(&self) -> &CompressedG1 {
        &self.g_c_prime
    }

    pub fn g_k(&self) -> &CompressedG1 {
        &self.g_k
    }

    pub fn g_h(&self) -> &CompressedG1 {
        &self.g_h
    }
}

impl Parse for PHGRProof {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            g_a: parse!(p, "g_a")?,
            g_a_prime: parse!(p, "g_a_prime")?,
            g_b: parse!(p, "g_b")?,
            g_b_prime: parse!(p, "g_b_prime")?,
            g_c: parse!(p, "g_c")?,
            g_c_prime: parse!(p, "g_c_prime")?,
            g_k: parse!(p, "g_k")?,
            g_h: parse!(p, "g_h")?,
        })
    }
}
