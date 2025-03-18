use std::collections::HashMap;

use crate::{Blob, Data};
use bc_envelope::prelude::*;

use super::{Anchor, Attachments};

/// For legacy Sprout transactions: JoinSplit descriptions that mix transparent and shielded values.
#[derive(Debug, Clone)]
pub struct JoinSplitDescription {
    anchor: Anchor,
    nullifiers: [Blob<32>; 2],
    commitments: [Blob<32>; 2],
    /// A zero-knowledge proof to validate the JoinSplit operation.
    zkproof: Data,
    // Further fields may be added as necessary.
    attachments: Attachments,
}
