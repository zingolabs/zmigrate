use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

use crate::Data;

use super::Attachments;

/// Details specific to shielded addresses.
#[derive(Debug, Clone)]
pub struct ShieldedAddress {
    /// The actual address string (could encode Sapling, Orchard, etc.).
    address: String, // Unique
    /// Optional diversifier or other Zcash-specific metadata.
    diversifier: Option<Data>,
    attachments: Attachments,
}
