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
        Ok(Self {
            tree: parse!(p, "tree")?,
            filled: parse!(p, "filled")?,
            cursor: parse!(p, "cursor")?,
        })
    }
}
