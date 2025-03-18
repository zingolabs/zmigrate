use std::collections::HashMap;

use bc_components::ARID;

use super::{Account, Attachments, Identifiable, SeedMaterial};

/// Represents an entire wallet, including multiple accounts, a wallet-specific
/// subset of the global transaction history, and optionally a form of seed
/// material for generating cryptographic keys.
///
/// This is *not* the top level of the interchange format hierarchy. That is
/// the `WalletDB` type.
#[derive(Debug, Clone)]
pub struct Wallet {
    pub id: ARID,
    pub seed_material: Option<SeedMaterial>,
    pub accounts: HashMap<ARID, Account>,
    pub attachments: Attachments,
}

/// Further impls of this omitted for brevity.
impl Identifiable for Wallet {
    fn id(&self) -> &ARID {
        &self.id
    }
}
