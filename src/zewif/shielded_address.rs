use crate::Data;

use super::Attachments;

/// Details specific to shielded addresses.
#[derive(Debug, Clone)]
pub struct ShieldedAddress {
    /// The actual address string (could encode Sapling, Orchard, etc.).
    pub address: String, // Unique
    /// Optional diversifier or other Zcash-specific metadata.
    pub diversifier: Option<Data>,
    pub attachments: Attachments,
}
