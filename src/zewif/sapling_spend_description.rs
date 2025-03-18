use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::Envelope;

use crate::{Amount, Blob, BlockHeight, Data};

use super::Attachments;

/// Data specific to Sapling spends.
#[derive(Debug, Clone)]
pub struct SaplingSpendDescription {
    spend_index: u32,
    /// The value of the input note, if known.
    value: Option<Amount>,
    /// The height that the anchor corresponds to, if known.
    anchor_height: Option<BlockHeight>,
    /// A nullifier to ensure the note is spent only once.
    nullifier: Blob<32>,
    /// A zero-knowledge proof that the spend is valid.
    zkproof: Data,
    // Additional fields (e.g., spending key components) may be required.
    attachments: Attachments,
}
