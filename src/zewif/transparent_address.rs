use super::{Attachments, DerivationInfo, TransparentSpendAuthority};

#[derive(Debug, Clone)]
pub struct TransparentAddress {
    pub address: String, // Unique
    pub spend_authority: Option<TransparentSpendAuthority>,
    pub derivation_info: Option<DerivationInfo>,
    pub attachments: Attachments,
}
