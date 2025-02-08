use anyhow::{ Result, Context };

use crate::{ Blob, Parse, Parser };

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

impl Parse for PHGRProof {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let g_a = Parse::parse(parser).context("g_a")?;
        let g_a_prime = Parse::parse(parser).context("g_a_prime")?;
        let g_b = Parse::parse(parser).context("g_b")?;
        let g_b_prime = Parse::parse(parser).context("g_b_prime")?;
        let g_c = Parse::parse(parser).context("g_c")?;
        let g_c_prime = Parse::parse(parser).context("g_c_prime")?;
        let g_k = Parse::parse(parser).context("g_k")?;
        let g_h = Parse::parse(parser).context("g_h")?;
        Ok(Self {
            g_a,
            g_a_prime,
            g_b,
            g_b_prime,
            g_c,
            g_c_prime,
            g_k,
            g_h,
        })
    }
}
