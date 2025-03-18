use crate::{Blob, Data};

use super::{Anchor, Attachments};

/// For legacy Sprout transactions: JoinSplit descriptions that mix transparent and shielded values.
#[derive(Debug, Clone)]
pub struct JoinSplitDescription {
    pub anchor: Anchor,
    pub nullifiers: [Blob<32>; 2],
    pub commitments: [Blob<32>; 2],
    /// A zero-knowledge proof to validate the JoinSplit operation.
    pub zkproof: Data,
    // Further fields may be added as necessary.
    pub attachments: Attachments,
}
