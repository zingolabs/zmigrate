// #![allow(dead_code, unused_imports)]

mod account; pub use account::*;
mod address; pub use address::*;
mod amount; pub use amount::*;
mod anchor; pub use anchor::*;
mod attachments; pub use attachments::*;
mod blob_macro;
mod blob; pub use blob::*;
mod block_height; pub use block_height::*;
mod compact_size; pub use compact_size::*;
mod data_macro;
mod data; pub use data::*;
mod derivation_info; pub use derivation_info::*;
mod identifiable; pub use identifiable::*;
mod incremental_merkle_tree; pub use incremental_merkle_tree::*;
mod incremental_witness; pub use incremental_witness::*;
mod join_split_description; pub use join_split_description::*;
mod orchard_action_description; pub use orchard_action_description::*;
mod orchard_sent_output; pub use orchard_sent_output::*;
mod parseable_types; pub use parseable_types::*;
mod position; pub use position::*;
mod sapling_output_description; pub use sapling_output_description::*;
mod sapling_sent_output; pub use sapling_sent_output::*;
mod sapling_spend_description; pub use sapling_spend_description::*;
mod sapling_witness; pub use sapling_witness::*;
mod seconds_since_epoch; pub use seconds_since_epoch::*;
mod seed_material; pub use seed_material::*;
mod shielded_address; pub use shielded_address::*;
mod spending_key; pub use spending_key::*;
mod sprout_witness; pub use sprout_witness::*;
mod string_macro;
mod transaction; pub use transaction::*;
mod transparent_address; pub use transparent_address::*;
mod transparent_spend_authority; pub use transparent_spend_authority::*;
mod tx_in; pub use tx_in::*;
mod tx_out_point; pub use tx_out_point::*;
mod tx_out; pub use tx_out::*;
mod txid; pub use txid::*;
mod u160_type; pub use u160_type::*;
mod u252_type; pub use u252_type::*;
mod u256_type; pub use u256_type::*;
mod wallet; pub use wallet::*;

use std::collections::HashMap;

use bc_components::ARID;

/// Represents a wallet database, the top level of the interchange format
/// hierarchy, which can contain multiple wallets and a global transaction
/// history.
#[derive(Debug, Clone)]
pub struct ZewifWallets {
    pub wallets: HashMap<ARID, Wallet>,
    pub transactions: HashMap<TxId, Transaction>,
    pub attachments: Attachments,
}
