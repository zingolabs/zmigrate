mod sapling_bundle; pub use sapling_bundle::*;
mod sapling_incoming_viewing_key; pub use sapling_incoming_viewing_key::*;
mod sapling_note_data; pub use sapling_note_data::*;

use crate::u256;

use super::IncrementalWitness;

const SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
pub type PedersenHash = u256;
pub type SaplingWitness = IncrementalWitness<SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH, PedersenHash>;
