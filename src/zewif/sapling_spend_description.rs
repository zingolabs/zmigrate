use crate::{Amount, Blob, BlockHeight, Data};

use super::Attachments;

/// Data specific to Sapling spends.
#[derive(Debug, Clone)]
pub struct SaplingSpendDescription {
    pub spend_index: u32,
    /// The value of the input note, if known.
    pub value: Option<Amount>,
    /// The height that the anchor corresponds to, if known.
    pub anchor_height: Option<BlockHeight>,
    /// A nullifier to ensure the note is spent only once.
    pub nullifier: Blob<32>,
    /// A zero-knowledge proof that the spend is valid.
    pub zkproof: Data,
    // Additional fields (e.g., spending key components) may be required.
    pub attachments: Attachments,
}
