use anyhow::Result;

use crate::{ parse, Blob, Parse, Parser };

// Initial bool || Fq
pub type CompressedG1 = Blob<33>;

#[derive(Debug, Clone, PartialEq)]
pub struct PHGRProof {
    pub g_a: CompressedG1,
    pub g_a_prime: CompressedG1,
    pub g_b: CompressedG1,
    pub g_b_prime: CompressedG1,
    pub g_c: CompressedG1,
    pub g_c_prime: CompressedG1,
    pub g_k: CompressedG1,
    pub g_h: CompressedG1,
}

impl Parse for PHGRProof {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            g_a:        parse!(p, "g_a")?,
            g_a_prime:  parse!(p, "g_a_prime")?,
            g_b:        parse!(p, "g_b")?,
            g_b_prime:  parse!(p, "g_b_prime")?,
            g_c:        parse!(p, "g_c")?,
            g_c_prime:  parse!(p, "g_c_prime")?,
            g_k:        parse!(p, "g_k")?,
            g_h:        parse!(p, "g_h")?,
        })
    }
}
