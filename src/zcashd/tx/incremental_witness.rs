use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

use super::IncrementalMerkleTree;

pub type SproutWitness = IncrementalWitness;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness {
    tree: IncrementalMerkleTree,
    filled: Vec<u256>,
    cursor: Option<IncrementalMerkleTree>,
}

impl IncrementalWitness {
    pub fn tree(&self) -> &IncrementalMerkleTree {
        &self.tree
    }

    pub fn filled(&self) -> &[u256] {
        &self.filled
    }

    pub fn cursor(&self) -> Option<&IncrementalMerkleTree> {
        self.cursor.as_ref()
    }
}

impl Parse for IncrementalWitness {
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
