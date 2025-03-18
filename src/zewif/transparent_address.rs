use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

use super::{Attachments, DerivationInfo, TransparentSpendAuthority};

#[derive(Debug, Clone)]
pub struct TransparentAddress {
    address: String, // Unique
    spend_authority: Option<TransparentSpendAuthority>,
    derivation_info: Option<DerivationInfo>,
    attachments: Attachments,
}
