use anyhow::Result;

use crate::{IncrementalMerkleTree, Parse, Parser, parse};

impl Parse for IncrementalMerkleTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        let left = parse!(p, "left")?;
        let right = parse!(p, "right")?;
        let parents = parse!(p, "parents")?;
        Ok(Self::with_fields(left, right, parents))
    }
}
