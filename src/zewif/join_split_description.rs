use crate::{Data, impl_attachable, u256};

use super::{Anchor, Attachments};

/// For legacy Sprout transactions: JoinSplit descriptions that mix transparent and shielded values.
#[derive(Debug, Clone)]
pub struct JoinSplitDescription {
    pub anchor: Anchor,
    pub nullifiers: [u256; 2],
    pub commitments: [u256; 2],
    /// A zero-knowledge proof to validate the JoinSplit operation.
    pub zkproof: Data,
    // Further fields may be added as necessary.
    pub attachments: Attachments,
}

impl_attachable!(JoinSplitDescription);
