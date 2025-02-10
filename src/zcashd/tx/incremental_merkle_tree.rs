use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalMerkleTree {
    pub left: Option<u256>,
    pub right: Option<u256>,
    pub parents: Vec<Option<u256>>,
}

impl Parse for IncrementalMerkleTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        let left = parse!(p, "incremental merkle tree left")?;
        let right = parse!(p, "incremental merkle tree right")?;
        let parents = parse!(p, "incremental merkle tree parents")?;
        Ok(Self {
            left,
            right,
            parents,
        })
    }
}
