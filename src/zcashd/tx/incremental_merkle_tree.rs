use anyhow::Result;

use crate::{IncrementalMerkleTree, Parse, Parser, parse};

impl Parse for IncrementalMerkleTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            left: parse!(p, "left")?,
            right: parse!(p, "right")?,
            parents: parse!(p, "parents")?,
        })
    }
}
