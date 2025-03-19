use crate::Data;

use super::SaplingIncomingViewingKey;

/// Details specific to shielded addresses.
#[derive(Debug, Clone)]
pub struct ShieldedAddress {
    /// The actual address string (could encode Sapling, Orchard, etc.).
    address: String, // Unique
    incoming_viewing_key: Option<SaplingIncomingViewingKey>,
    /// Optional diversifier or other Zcash-specific metadata.
    diversifier: Option<Data>,
}

impl ShieldedAddress {
    pub fn new(address: String) -> Self {
        ShieldedAddress {
            address,
            incoming_viewing_key: None,
            diversifier: None,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn incoming_viewing_key(&self) -> Option<&SaplingIncomingViewingKey> {
        self.incoming_viewing_key.as_ref()
    }

    pub fn set_incoming_viewing_key(&mut self, ivk: SaplingIncomingViewingKey) {
        self.incoming_viewing_key = Some(ivk);
    }

    pub fn diversifier(&self) -> Option<&Data> {
        self.diversifier.as_ref()
    }

    pub fn set_diversifier(&mut self, diversifier: Data) {
        self.diversifier = Some(diversifier);
    }
}
