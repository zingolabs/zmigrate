use crate::impl_attachable;

use super::{Attachments, ShieldedAddress, TransparentAddress};

#[derive(Debug, Clone)]
pub struct Address {
    name: String,
    address: ProtocolAddress,
    attachments: Attachments,
}

impl_attachable!(Address);

impl Address {
    pub fn new(address: ProtocolAddress) -> Self {
        Self {
            name: String::default(),
            address,
            attachments: Attachments::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn as_string(&self) -> String {
        self.address.as_string()
    }

    pub fn address(&self) -> &ProtocolAddress {
        &self.address
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_address(&mut self, address: ProtocolAddress) {
        self.address = address;
    }
}

/// An address can be either an exposed transparent address or one of several shielded types.
#[derive(Debug, Clone)]
pub enum ProtocolAddress {
    /// An exposed transparent (T-address) similar to Bitcoin's.
    Transparent(TransparentAddress),
    /// A shielded address (Z-address). This can include Sapling, Sprout, or Orchard formats.
    Shielded(ShieldedAddress),
}

impl ProtocolAddress {
    pub fn as_string(&self) -> String {
        match self {
            ProtocolAddress::Transparent(addr) => addr.address().to_string(),
            ProtocolAddress::Shielded(addr) => addr.address().to_string(),
        }
    }
}
