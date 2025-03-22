use crate::Parse;
use crate::{Data, Parser, u256, zewif::Position};
use anyhow::{Context, Result, bail};
use byteorder::{ByteOrder, LittleEndian};
use std::collections::HashMap;

/// Represents a node in the Orchard note commitment tree
#[derive(Debug, Clone, PartialEq)]
pub struct NoteCommitmentTreeNode {
    pub hash: u256,
    pub left: Option<Box<NoteCommitmentTreeNode>>,
    pub right: Option<Box<NoteCommitmentTreeNode>>,
    /// The position of this node in the tree (zero-based index)
    pub position: usize,
}

/// Represents the complete Orchard note commitment tree
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OrchardNoteCommitmentTree {
    pub unparsed_data: Data,
    pub root: Option<NoteCommitmentTreeNode>,
    pub tree_size: u64,
    pub nodes: Vec<Option<u256>>,
    pub depth: usize,
    /// Maps note commitments to their positions in the tree
    pub commitment_positions: HashMap<u256, Position>,
}

impl OrchardNoteCommitmentTree {
    /// Create a new empty tree
    pub fn new() -> Self {
        Self {
            unparsed_data: Data(Vec::new()),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        }
    }

    /// Parse the raw tree data into a structured format with detailed error handling
    pub fn parse_tree_data(&mut self) -> Result<()> {
        if self.unparsed_data.0.is_empty() {
            return Ok(());
        }

        let data = &self.unparsed_data.0;

        // Validate minimum data length for header (4 + 8 + 1 = 13 bytes)
        if data.len() < 13 {
            bail!("Invalid tree data: too short for header (got {} bytes, need at least 13)", data.len());
        }

        // Check the tree format version - first 4 bytes
        let format_version = LittleEndian::read_u32(&data[0..4]);
        if format_version != 1 {
            bail!("Unsupported tree format version: {}", format_version);
        }

        // Parse tree size (number of notes) - next 8 bytes
        let tree_size = LittleEndian::read_u64(&data[4..12]);
        self.tree_size = tree_size;

        // The depth of the tree - 1 byte
        let depth = data[12] as usize;
        self.depth = depth;

        // Validate reasonable tree depth (protects against memory allocation attacks)
        if depth > 64 {
            bail!("Invalid tree depth: {} (maximum supported is 64)", depth);
        }

        // The rest of the data is the serialized tree structure
        // A valid tree should have at least (2^depth - 1) possible node positions
        let expected_node_count = (1 << depth) - 1;

        // Calculate expected data size based on node count
        // Each node needs 1 byte for presence flag + up to 32 bytes for the hash
        let _min_expected_data_size = 13; // Header size
        let max_expected_data_size = 13 + expected_node_count * 33; // Header + max possible node data

        if data.len() > max_expected_data_size {
            bail!("Invalid tree data: too large (got {} bytes, expected at most {})",
                  data.len(), max_expected_data_size);
        }

        self.nodes = Vec::with_capacity(expected_node_count);

        // Parse the node structure
        let mut position = 13; // starting after header
        let mut node_count = 0;

        while position < data.len() && node_count < expected_node_count {
            // Each node entry starts with a flag byte indicating if it's present
            if position >= data.len() {
                bail!("Invalid tree data: truncated at node presence flag");
            }

            let has_node = data[position] != 0;
            position += 1;

            if has_node {
                // Read the 32-byte node hash
                if position + 32 <= data.len() {
                    let node_hash = u256::from_slice(&data[position..position + 32])
                        .context("Failed to parse node hash")?;
                    self.nodes.push(Some(node_hash));
                    position += 32;
                } else {
                    bail!("Invalid tree data: truncated node hash at position {}", position);
                }
            } else {
                // No node at this position
                self.nodes.push(None);
            }

            node_count += 1;
        }

        // Validate node count
        if node_count == 0 {
            // Empty tree is valid but unusual, log a warning
            eprintln!("Warning: Parsed an empty note commitment tree");
        } else if node_count < expected_node_count {
            // We didn't get all expected nodes, but we might have a valid partial tree
            eprintln!("Warning: Parsed a partial tree with {} nodes (expected {})", node_count, expected_node_count);
        }

        // Reconstruct the tree structure
        if !self.nodes.is_empty() {
            self.root = self.build_tree_node(0, 0); // Start at index 0, position 0
            self.build_commitment_position_map();
        }

        Ok(())
    }

    /// Recursively build the tree structure from the flat nodes array
    fn build_tree_node(&self, index: usize, position: usize) -> Option<NoteCommitmentTreeNode> {
        if index >= self.nodes.len() {
            return None;
        }

        if let Some(hash) = self.nodes[index] {
            // Calculate left and right child indices
            let left_idx = 2 * index + 1;
            let right_idx = 2 * index + 2;

            // Calculate left and right child positions
            let left_pos = 2 * position + 1;
            let right_pos = 2 * position + 2;

            let left = if left_idx < self.nodes.len() {
                self.build_tree_node(left_idx, left_pos).map(Box::new)
            } else {
                None
            };

            let right = if right_idx < self.nodes.len() {
                self.build_tree_node(right_idx, right_pos).map(Box::new)
            } else {
                None
            };

            Some(NoteCommitmentTreeNode {
                hash,
                left,
                right,
                position,
            })
        } else {
            None
        }
    }

    /// Build a mapping between commitments and their positions in the tree
    fn build_commitment_position_map(&mut self) {
        self.commitment_positions.clear();

        // Clone the root to avoid borrowing issues
        if let Some(root) = self.root.clone() {
            // Collect all commitments from the tree recursively
            let mut commitments = Vec::new();
            Self::collect_commitments_recursive(&root, &mut commitments);

            // Add all collected commitments to the map
            for (hash, position) in commitments {
                self.commitment_positions.insert(hash, Position(position as u32));
            }
        }
    }

    /// Recursively collect commitments and their positions without mutating self
    fn collect_commitments_recursive(node: &NoteCommitmentTreeNode, results: &mut Vec<(u256, usize)>) {
        // Add this node's commitment and position
        results.push((node.hash, node.position));

        // Process left child if present
        if let Some(left) = &node.left {
            Self::collect_commitments_recursive(left, results);
        }

        // Process right child if present
        if let Some(right) = &node.right {
            Self::collect_commitments_recursive(right, results);
        }
    }

    /// Find the position of a commitment in the tree
    pub fn find_position(&self, commitment: &u256) -> Option<Position> {
        self.commitment_positions.get(commitment).copied()
    }

    /// Convert to Zewif IncrementalMerkleTree format
    pub fn to_zewif_tree(&self) -> crate::zewif::IncrementalMerkleTree {
        let mut tree = crate::zewif::IncrementalMerkleTree::new();

        // Convert the root node
        if let Some(root_node) = &self.root {
            // The root node's left and right children are the first level
            if let Some(left) = &root_node.left {
                tree.left = Some(left.hash);
            }

            if let Some(right) = &root_node.right {
                tree.right = Some(right.hash);
            }

            // Add parents (ancestors) from the tree
            // In a full implementation, we need to add all parent nodes in the correct order
            // to properly represent the tree structure
            let mut parents = Vec::with_capacity(self.depth.saturating_sub(1));

            // For each level of the tree (excluding the root and the bottom level)
            for level in 0..self.depth.saturating_sub(1) {
                // Calculate the starting index for this level in a complete binary tree
                let start_idx = (1 << level) - 1;

                // For a complete level, we'd have 2^level nodes
                let level_size = 1 << level;

                // Collect all nodes from this level
                let mut level_nodes = Vec::with_capacity(level_size);

                for i in 0..level_size {
                    let node_idx = start_idx + i;
                    if node_idx < self.nodes.len() {
                        level_nodes.push(self.nodes[node_idx]);
                    } else {
                        level_nodes.push(None);
                    }
                }

                // Add all nodes from this level to the parents vector
                parents.extend(level_nodes);
            }

            tree.parents = parents;
        }

        tree
    }
    
    /// Create a witness for a commitment that exists in the tree.
    /// 
    /// This method:
    /// 1. Verifies that the commitment exists in the tree
    /// 2. Converts the tree structure to the ZeWIF format
    /// 3. Creates a witness object containing the tree state
    /// 4. Returns both the anchor (root hash) and the witness
    /// 
    /// The witness data is essential for proving that a note exists in the commitment tree,
    /// which is required for spending notes in ZCash's shielded pools.
    /// 
    /// Returns None if the commitment is not found in the tree.
    pub fn create_witness(&self, commitment: &u256) -> Option<(crate::zewif::Anchor, crate::zewif::OrchardWitness)> {
        // Find the position of the commitment in the tree - needed to make sure the commitment exists
        self.find_position(commitment)?;
        
        // Convert the tree to Zewif format
        let zewif_tree = self.to_zewif_tree();
        
        // Create a witness using the tree
        let witness = crate::zewif::OrchardWitness::from_tree(zewif_tree);
        
        // Get the root to use as an anchor
        if let Some(root_node) = &self.root {
            let anchor = root_node.hash;
            return Some((anchor, witness));
        }
        
        None
    }
}

impl Parse for OrchardNoteCommitmentTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        let mut tree = Self {
            unparsed_data: p.rest(),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        };

        // Parse the tree data immediately during construction
        // We'll ignore errors here - if we can't parse it, we'll have the unparsed data
        // but we'll log the error to help with debugging
        if let Err(e) = tree.parse_tree_data() {
            eprintln!("Warning: Failed to parse orchard note commitment tree: {}", e);
        }

        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Data;
    use byteorder::{ByteOrder, LittleEndian};

    // Helper function to create a mock tree for testing
    fn create_mock_tree_data(depth: u8, num_nodes: usize) -> Vec<u8> {
        let mut data = Vec::new();

        // Format version (4 bytes, LE)
        let version: u32 = 1;
        let mut version_bytes = [0u8; 4];
        LittleEndian::write_u32(&mut version_bytes, version);
        data.extend_from_slice(&version_bytes);

        // Tree size (8 bytes, LE)
        let tree_size: u64 = num_nodes as u64;
        let mut size_bytes = [0u8; 8];
        LittleEndian::write_u64(&mut size_bytes, tree_size);
        data.extend_from_slice(&size_bytes);

        // Depth (1 byte)
        data.push(depth);

        // Add node data
        for i in 0..num_nodes {
            // Node presence flag (1 = present)
            data.push(1);

            // Node hash (32 bytes)
            let mut hash = [0u8; 32];
            // Just put the index in the first byte to make each hash unique
            hash[0] = (i % 255) as u8;
            data.extend_from_slice(&hash);
        }

        data
    }

    #[test]
    fn test_tree_parsing() {
        // Create a mock tree with depth 3 and 7 nodes (a complete binary tree would have 7 nodes)
        let mock_data = create_mock_tree_data(3, 7);

        // Create a tree with this data
        let mut tree = OrchardNoteCommitmentTree {
            unparsed_data: Data(mock_data),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        };

        // Parse the tree data
        let result = tree.parse_tree_data();
        assert!(result.is_ok(), "Tree parsing should succeed");

        // Verify the parsed data
        assert_eq!(tree.depth, 3, "Tree depth should be 3");
        assert_eq!(tree.tree_size, 7, "Tree size should be 7");
        assert_eq!(tree.nodes.len(), 7, "There should be 7 nodes");

        // Check that all nodes are present (Some)
        for node in &tree.nodes {
            assert!(node.is_some(), "All nodes should be present");
        }

        // Verify the root node was constructed
        assert!(tree.root.is_some(), "Root node should be constructed");

        // Check the commitment position mapping
        assert_eq!(tree.commitment_positions.len(), 7, "All nodes should have positions");

        // Verify we can find positions by commitment
        if let Some(root_node) = &tree.root {
            let position = tree.find_position(&root_node.hash);
            assert_eq!(position, Some(Position(0)), "Root should be at position 0");

            // Check left child if it exists
            if let Some(left_child) = &root_node.left {
                let position = tree.find_position(&left_child.hash);
                assert_eq!(position, Some(Position(1)), "Left child should be at position 1");
            }

            // Check right child if it exists
            if let Some(right_child) = &root_node.right {
                let position = tree.find_position(&right_child.hash);
                assert_eq!(position, Some(Position(2)), "Right child should be at position 2");
            }
        }
    }

    #[test]
    fn test_error_handling() {
        // Test with too small data (less than header)
        let too_small = vec![1, 2, 3]; // Only 3 bytes
        let mut tree = OrchardNoteCommitmentTree {
            unparsed_data: Data(too_small),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        };

        let result = tree.parse_tree_data();
        assert!(result.is_err(), "Parsing too small data should fail");

        // Test with invalid version
        let mut invalid_version = create_mock_tree_data(3, 7);
        // Change version to 2 (unsupported)
        invalid_version[0] = 2;

        let mut tree = OrchardNoteCommitmentTree {
            unparsed_data: Data(invalid_version),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        };

        let result = tree.parse_tree_data();
        assert!(result.is_err(), "Parsing invalid version should fail");

        // Test with truncated node data
        let mut truncated = create_mock_tree_data(3, 7);
        // Remove the last few bytes to truncate a node
        truncated.truncate(truncated.len() - 10);

        let mut tree = OrchardNoteCommitmentTree {
            unparsed_data: Data(truncated),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        };

        let result = tree.parse_tree_data();
        assert!(result.is_err(), "Parsing truncated data should fail");
    }

    #[test]
    fn test_zewif_tree_conversion() {
        // Create a mock tree
        let mock_data = create_mock_tree_data(3, 7);

        // Create and parse a tree
        let mut tree = OrchardNoteCommitmentTree {
            unparsed_data: Data(mock_data),
            root: None,
            tree_size: 0,
            nodes: Vec::new(),
            depth: 0,
            commitment_positions: HashMap::new(),
        };

        let result = tree.parse_tree_data();
        assert!(result.is_ok(), "Tree parsing should succeed");

        // Convert to Zewif format
        let zewif_tree = tree.to_zewif_tree();

        // Verify the conversion
        assert!(zewif_tree.left.is_some(), "Zewif tree should have left node");
        assert!(zewif_tree.right.is_some(), "Zewif tree should have right node");

        // Our improved implementation includes all levels from 0 to depth-2
        // For depth=3, we have nodes from levels 0, 1 (total of 3 nodes: 1+2)
        assert_eq!(zewif_tree.parents.len(), 3, "Should have 3 parents for depth 3");
    }
}
