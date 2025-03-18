use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

use crate::{u256, Blob, Data};

use super::{Anchor, Attachments, IncrementalWitness, Position, SaplingWitness};

/// Data specific to Sapling outputs.
/// *this is currently not sufficient to make all outputs recoverable*
/// Additional things that are needed:
/// * recipient address (unified address that was specified as the recipient by the user)
/// * value of the output
/// * other information in https://github.com/zcash/librustzcash/blob/54fd075449218e27e4cbf5cb7108cd89f458acc0/zcash_client_sqlite/src/wallet/db.rs#L455 may not be recoverable from the chain
#[derive(Debug, Clone)]
pub struct SaplingOutputDescription {
    output_index: u32,
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
    witness: Option<(Anchor, SaplingWitness)>,
    attachments: Attachments,
}
