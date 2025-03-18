use std::collections::HashMap;

use bc_components::{Digest, ARID};
use bc_envelope::prelude::*;

use super::{Account, Attachments, Identifiable, SeedMaterial};

/// Represents an entire wallet, including multiple accounts, a wallet-specific
/// subset of the global transaction history, and optionally a form of seed
/// material for generating cryptographic keys.
///
/// This is *not* the top level of the interchange format hierarchy. That is
/// the `WalletDB` type.
#[derive(Debug, Clone)]
pub struct Wallet {
    id: ARID,
    seed_material: Option<SeedMaterial>,
    accounts: HashMap<ARID, Account>,
    attachments: Attachments,
}

/// Further impls of this omitted for brevity.
impl Identifiable for Wallet {
    fn id(&self) -> &ARID {
        &self.id
    }
}
