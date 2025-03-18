use std::collections::{HashMap, HashSet};

use bc_components::ARID;

use crate::TxId;

use super::{Address, Attachments, OrchardSentOutput, SaplingSentOutput};

/// Logical grouping within a wallet. Each account can have its own set of
/// addresses, transactions, and other metadata.
#[derive(Debug, Clone)]
pub struct Account {
    pub id: ARID,
    pub name: String, // May not be unique.
    pub zip32_account_id: Option<u32>,
    pub addresses: HashMap<String, Address>,
    pub relevant_transactions: HashSet<TxId>, // Subset of the global transaction history.
    // The following are intended for storage of information that may not be
    // recoverable from the chain.
    pub sapling_sent_outputs: Vec<SaplingSentOutput>,
    pub orchard_sent_outputs: Vec<OrchardSentOutput>,
    pub attachments: Attachments,
}
