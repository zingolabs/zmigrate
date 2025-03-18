use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

use crate::{u256, Blob, Data};

use super::{Anchor, Attachments, IncrementalWitness, Position};

const ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
pub type SinsemillaHash = u256;
pub type OrchardWitness = IncrementalWitness<ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH, SinsemillaHash>;

/// Data specific to Orchard actions.
/// TODO CHECK
#[derive(Debug, Clone)]
pub struct OrchardActionDescription {
    action_index: u32,
    /// The anchor of the current commitment tree.
    anchor: Blob<32>,
    /// A nullifier to ensure the note is spent only once.
    nullifier: Blob<32>,
    /// A zero-knowledge proof that the spend is valid.
    zkproof: Data,
    /// Additional fields (e.g., spending key components) may be required.
    /// The note commitment.
    commitment: Blob<32>,
    /// Ephemeral key for the encrypted note.
    ephemeral_key: Blob<32>,
    /// Encrypted ciphertext containing the note details.
    enc_ciphertext: Data,
    /// An optional memo field.
    memo: Option<Data>,
    /// This and the witness are recorded at export as of
    /// an anchor depth 20 blocks back from the chain tip, or the oldest possible witness at a lesser depth.
    note_commitment_tree_position: Position,
    /// Witness
    witness: Option<(Anchor, OrchardWitness)>,
    attachments: Attachments,
}
