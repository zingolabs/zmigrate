mod ed25519_signature; pub use ed25519_signature::*;
mod ed25519_verification_key; pub use ed25519_verification_key::*;
mod join_splits; pub use join_splits::*;
mod js_description; pub use js_description::*;
mod js_out_point; pub use js_out_point::*;
mod note_encryption_ciphertext; pub use note_encryption_ciphertext::*;
mod phgr_proof; pub use phgr_proof::*;
mod sprout_note_data; pub use sprout_note_data::*;
mod sprout_payment_address; pub use sprout_payment_address::*;
mod sprout_proof; pub use sprout_proof::*;

use crate::u256;

use super::IncrementalWitness;

const INCREMENTAL_MERKLE_TREE_DEPTH: usize = 29;
pub type SHA256Compress = u256;
pub type SproutWitness = IncrementalWitness<INCREMENTAL_MERKLE_TREE_DEPTH, SHA256Compress>;
