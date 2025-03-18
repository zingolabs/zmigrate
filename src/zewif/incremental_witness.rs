use super::IncrementalMerkleTree;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness<const DEPTH: usize, Hash> {
    pub tree: IncrementalMerkleTree,
    pub filled: Vec<Hash>,
    pub cursor: Option<IncrementalMerkleTree>,
}
