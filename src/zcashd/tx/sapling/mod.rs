mod output_v4; pub use output_v4::*;
mod output_v5; pub use output_v5::*;
mod sapling_bundle_v4; pub use sapling_bundle_v4::*;
mod sapling_bundle_v5; pub use sapling_bundle_v5::*;
mod sapling_bundle; pub use sapling_bundle::*;
mod sapling_expanded_spending_key; pub use sapling_expanded_spending_key::*;
mod sapling_extended_spending_key; pub use sapling_extended_spending_key::*;
mod sapling_incoming_viewing_key; pub use sapling_incoming_viewing_key::*;
mod sapling_key; pub use sapling_key::*;
mod sapling_keys; pub use sapling_keys::*;
mod sapling_note_data; pub use sapling_note_data::*;
mod sapling_z_payment_address; pub use sapling_z_payment_address::*;
mod spend_v4; pub use spend_v4::*;
mod spend_v5; pub use spend_v5::*;

use crate::u256;

use super::IncrementalWitness;

const SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
pub type PedersenHash = u256;
pub type SaplingWitness = IncrementalWitness<SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH, PedersenHash>;
