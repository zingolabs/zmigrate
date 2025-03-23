use super::IncrementalMerkleTree;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness<const DEPTH: usize, Hash> {
    tree: IncrementalMerkleTree,
    filled: Vec<Hash>,
    cursor: Option<IncrementalMerkleTree>,
}

impl<const DEPTH: usize, Hash> IncrementalWitness<DEPTH, Hash> {
    pub fn with_fields(
        tree: IncrementalMerkleTree,
        filled: Vec<Hash>,
        cursor: Option<IncrementalMerkleTree>,
    ) -> Self {
        Self {
            tree,
            filled,
            cursor,
        }
    }

    pub fn tree(&self) -> &IncrementalMerkleTree {
        &self.tree
    }

    pub fn filled(&self) -> &Vec<Hash> {
        &self.filled
    }

    pub fn cursor(&self) -> &Option<IncrementalMerkleTree> {
        &self.cursor
    }
}
