use anyhow::Result;

use crate::{ parse, Blob, Parse, Parser };

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
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        let g_a =       parse!(p, "g_a")?;
        let g_a_prime = parse!(p, "g_a_prime")?;
        let g_b =       parse!(p, "g_b")?;
        let g_b_prime = parse!(p, "g_b_prime")?;
        let g_c =       parse!(p, "g_c")?;
        let g_c_prime = parse!(p, "g_c_prime")?;
        let g_k =       parse!(p, "g_k")?;
        let g_h =       parse!(p, "g_h")?;
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
