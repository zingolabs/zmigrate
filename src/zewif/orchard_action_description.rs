use crate::{Data, impl_attachable, u256};

use super::{Anchor, Attachments, IncrementalWitness, Position};

const ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
pub type SinsemillaHash = u256;
pub type OrchardWitness = IncrementalWitness<ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH, SinsemillaHash>;

/// Data specific to Orchard actions.
#[derive(Debug, Clone)]
pub struct OrchardActionDescription {
    pub action_index: u32,
    /// The anchor of the current commitment tree.
    pub anchor: u256,
    /// A nullifier to ensure the note is spent only once.
    pub nullifier: u256,
    /// A zero-knowledge proof that the spend is valid.
    pub zkproof: Data,
    /// Additional fields (e.g., spending key components) may be required.
    /// The note commitment.
    pub commitment: u256,
    /// Ephemeral key for the encrypted note.
    pub ephemeral_key: u256,
    /// Encrypted ciphertext containing the note details.
    pub enc_ciphertext: Data,
    /// An optional memo field.
    pub memo: Option<Data>,
    /// This and the witness are recorded at export as of
    /// an anchor depth 20 blocks back from the chain tip, or the oldest possible witness at a lesser depth.
    pub note_commitment_tree_position: Position,
    /// Witness
    pub witness: Option<(Anchor, OrchardWitness)>,
    pub attachments: Attachments,
}

impl_attachable!(OrchardActionDescription);
