use super::RecipientAddress;

#[derive(Debug, Clone, PartialEq)]
pub struct RecipientMapping {
    recipient_address: RecipientAddress,
    unified_address: String,
}

impl RecipientMapping {
    pub fn recipient_address(&self) -> &RecipientAddress {
        &self.recipient_address
    }

    pub fn unified_address(&self) -> &str {
        &self.unified_address
    }
}

impl RecipientMapping {
    pub fn new(recipient_address: RecipientAddress, unified_address: String) -> Self {
        Self { recipient_address, unified_address }
    }
}
