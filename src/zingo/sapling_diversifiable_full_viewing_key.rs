use anyhow::Result;

use crate::{blob, parse, Blob, Parse, Parser};

blob!(SaplingFullViewingKey, 96);
blob!(SaplingDiversifierKey, 32);

#[derive(Debug, Clone)]
pub struct SaplingDiversifiableFullViewingKey {
    fvk: SaplingFullViewingKey,
    dk: SaplingDiversifierKey,
}

impl Parse for SaplingDiversifiableFullViewingKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let fvk = parse!(p, "SaplingFullViewingKey")?;
        let dk = parse!(p, "SaplingDiversifierKey")?;
        Ok(Self { fvk, dk })
    }
}
