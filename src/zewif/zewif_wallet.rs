use std::collections::HashMap;

use bc_components::ARID;

use crate::impl_attachable;

use super::{Account, Attachments, Identifiable, SeedMaterial};

/// Represents an entire wallet, including multiple accounts, a wallet-specific
/// subset of the global transaction history, and optionally a form of seed
/// material for generating cryptographic keys.
///
/// This is *not* the top level of the interchange format hierarchy. That is
/// the `ZewifDB` type.
#[derive(Debug, Clone)]
pub struct ZewifWallet {
    pub id: ARID,
    pub seed_material: Option<SeedMaterial>,
    pub accounts: HashMap<ARID, Account>,
    pub attachments: Attachments,
}

/// Further impls of this omitted for brevity.
impl Identifiable for ZewifWallet {
    fn id(&self) -> &ARID {
        &self.id
    }
}

impl_attachable!(ZewifWallet);

impl ZewifWallet {
    pub fn new() -> Self {
        Self {
            id: ARID::new(),
            seed_material: None,
            accounts: HashMap::new(),
            attachments: Attachments::new(),
        }
    }
}

impl Default for ZewifWallet {
    fn default() -> Self {
        Self::new()
    }
}
