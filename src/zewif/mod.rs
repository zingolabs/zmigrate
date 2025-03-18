// #![allow(dead_code, unused_imports)]

mod account; pub use account::*;
mod address; pub use address::*;
mod anchor; pub use anchor::*;
mod attachments; pub use attachments::*;
mod derivation_info; pub use derivation_info::*;
mod identifiable; pub use identifiable::*;
mod incremental_merkle_tree; pub use incremental_merkle_tree::*;
mod incremental_witness; pub use incremental_witness::*;
mod join_split_description; pub use join_split_description::*;
mod orchard_action_description; pub use orchard_action_description::*;
mod orchard_sent_output; pub use orchard_sent_output::*;
mod position; pub use position::*;
mod sapling_sent_output; pub use sapling_sent_output::*;
mod sapling_witness; pub use sapling_witness::*;
mod sapling_output_description; pub use sapling_output_description::*;
mod sapling_spend_description; pub use sapling_spend_description::*;
mod seed_material; pub use seed_material::*;
mod shielded_address; pub use shielded_address::*;
mod spending_key; pub use spending_key::*;
mod sprout_witness; pub use sprout_witness::*;
mod transaction; pub use transaction::*;
mod transparent_address; pub use transparent_address::*;
mod transparent_spend_authority; pub use transparent_spend_authority::*;
mod tx_in; pub use tx_in::*;
mod tx_out; pub use tx_out::*;
mod tx_out_point; pub use tx_out_point::*;
mod wallet; pub use wallet::*;

use std::collections::HashMap;

use bc_components::ARID;

use crate::TxId;

/// Represents a wallet database, the top level of the interchange format
/// hierarchy, which can contain multiple wallets and a global transaction
/// history.
#[derive(Debug, Clone)]
pub struct WalletDB {
    pub wallets: HashMap<ARID, Wallet>,
    pub transactions: HashMap<TxId, Transaction>,
    pub attachments: Attachments,
}
