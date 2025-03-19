use crate::{impl_attachable, Data, u256};

use super::{Anchor, Attachments, Position, SaplingWitness};

/// Data specific to Sapling outputs.
/// *this is currently not sufficient to make all outputs recoverable*
/// Additional things that are needed:
/// * recipient address (unified address that was specified as the recipient by the user)
/// * value of the output
/// * other information in https://github.com/zcash/librustzcash/blob/54fd075449218e27e4cbf5cb7108cd89f458acc0/zcash_client_sqlite/src/wallet/db.rs#L455 may not be recoverable from the chain
#[derive(Debug, Clone)]
pub struct SaplingOutputDescription {
    pub output_index: u32,
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
    pub witness: Option<(Anchor, SaplingWitness)>,
    pub attachments: Attachments,
}

impl_attachable!(SaplingOutputDescription);
