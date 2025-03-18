use crate::u256;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalMerkleTree {
    pub left: Option<u256>,
    pub right: Option<u256>,
    pub parents: Vec<Option<u256>>,
}
