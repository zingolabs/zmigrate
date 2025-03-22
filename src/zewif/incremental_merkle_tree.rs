use crate::{u256, Position};

/// A Merkle tree that can be incrementally updated.
/// This structure represents a subtree of a larger Merkle tree,
/// and can be used to construct authentication paths (witnesses)
/// for leaves of the tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalMerkleTree {
    /// The left child of the current subtree root
    pub left: Option<u256>,
    /// The right child of the current subtree root
    pub right: Option<u256>,
    /// The parent nodes of the current subtree, ordered from lowest to highest
    pub parents: Vec<Option<u256>>,
}

impl IncrementalMerkleTree {
    /// Create a new empty incremental Merkle tree
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            parents: Vec::new(),
        }
    }
    
    /// Returns true if the tree is empty (has no nodes)
    pub fn is_empty(&self) -> bool {
        self.left.is_none() && self.right.is_none() && self.parents.is_empty()
    }
    
    /// Get the current position in the tree where the next leaf will be inserted
    pub fn position(&self) -> Position {
        // The position is determined by the structure of the tree
        // For a properly implemented tree, this would be based on the
        // number of leaves already in the tree
        // This is a simple implementation that doesn't account for tree state
        Position(0)
    }
    
    /// Get the current root of the tree
    pub fn root(&self) -> Option<u256> {
        if self.is_empty() {
            None
        } else if self.right.is_none() {
            self.left
        } else {
            // In a real implementation, we'd compute the root by combining left and right
            // with the parents array
            // For now, we'll just return the right node as a placeholder
            self.right
        }
    }
    
    /// Get the authentication path for a leaf at the given position
    /// Returns a vector of sibling nodes from bottom to top
    pub fn authentication_path(&self, position: Position) -> Vec<Option<u256>> {
        // In a full implementation, this would calculate the proper authentication path
        // For now, we're returning a minimal placeholder
        let mut path = Vec::new();
        
        // Add the first level sibling
        if position.0 % 2 == 0 {
            // Even positions have siblings to the right
            path.push(self.right);
        } else {
            // Odd positions have siblings to the left
            path.push(self.left);
        }
        
        // Add parent siblings
        for parent in &self.parents {
            path.push(*parent);
        }
        
        path
    }
}

impl Default for IncrementalMerkleTree {
    fn default() -> Self {
        Self::new()
    }
}