use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalMerkleTree {
    left: Option<u256>,
    right: Option<u256>,
    parents: Vec<Option<u256>>,
}

impl IncrementalMerkleTree {
    pub fn left(&self) -> Option<&u256> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&u256> {
        self.right.as_ref()
    }

    pub fn parents(&self) -> &[Option<u256>] {
        &self.parents
    }
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
