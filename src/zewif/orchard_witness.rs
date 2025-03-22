use crate::u256;

use super::IncrementalWitness;

/// The depth of the Orchard note commitment tree
pub const ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
/// The hash type used in the Orchard note commitment tree
pub type SinsemillaHash = u256;
/// Witness for an Orchard note commitment
pub type OrchardWitness = IncrementalWitness<ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH, SinsemillaHash>;