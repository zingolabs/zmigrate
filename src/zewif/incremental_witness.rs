use super::IncrementalMerkleTree;
use crate::u256;

/// A witness to a commitment in a note commitment tree.
/// The witness contains the information needed to verify that a commitment
/// exists in the tree at a specific position.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness<const DEPTH: usize, Hash> {
    /// The state of the tree at the time the witness was created
    pub tree: IncrementalMerkleTree,
    /// The filled nodes in the authentication path
    pub filled: Vec<Hash>,
    /// The cursor for incremental updates
    pub cursor: Option<IncrementalMerkleTree>,
}

impl<const DEPTH: usize, Hash> IncrementalWitness<DEPTH, Hash> {
    /// Create a new empty incremental witness
    pub fn new() -> Self {
        Self {
            tree: IncrementalMerkleTree::new(),
            filled: Vec::new(),
            cursor: None,
        }
    }
    
    /// Create a witness from a tree
    pub fn from_tree(tree: IncrementalMerkleTree) -> Self {
        Self {
            tree,
            filled: Vec::new(),
            cursor: None,
        }
    }
    
    /// Get the root of the tree at the time the witness was created
    pub fn root(&self) -> Option<u256> {
        self.tree.root()
    }
}

impl<const DEPTH: usize, Hash> Default for IncrementalWitness<DEPTH, Hash> {
    fn default() -> Self {
        Self::new()
    }
}
