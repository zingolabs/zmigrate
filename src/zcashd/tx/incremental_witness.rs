use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::IncrementalMerkleTree;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness<const DEPTH: usize, Hash> {
    pub tree: IncrementalMerkleTree,
    pub filled: Vec<Hash>,
    pub cursor: Option<IncrementalMerkleTree>,
}

impl<const DEPTH: usize, Hash: Parse> Parse for IncrementalWitness<DEPTH, Hash> {
    fn parse(p: &mut Parser) -> Result<Self> {
        let tree = parse!(p, "incremental witness tree")?;
        let filled = parse!(p, "incremental witness filled")?;
        let cursor = parse!(p, "incremental witness cursor")?;
        Ok(Self {
            tree,
            filled,
            cursor,
        })
    }
}
