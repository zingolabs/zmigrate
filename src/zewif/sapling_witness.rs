use crate::u256;

use super::IncrementalWitness;

const SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
pub type PedersenHash = u256;
pub type SaplingWitness = IncrementalWitness<SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH, PedersenHash>;
