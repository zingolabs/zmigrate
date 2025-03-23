use crate::mod_use;

mod blob_macro;
mod data_macro;
mod impl_attachable_macro;
mod string_macro;

mod_use!(account);
mod_use!(address_id);
mod_use!(address);
mod_use!(amount);
mod_use!(anchor);
mod_use!(attachments);
mod_use!(bip39_mnemonic);
mod_use!(blob);
mod_use!(block_height);
mod_use!(branch_id);
mod_use!(compact_size);
mod_use!(data);
mod_use!(derivation_info);
mod_use!(expiry_height);
mod_use!(groth_proof);
mod_use!(identifiable);
mod_use!(incremental_merkle_tree);
mod_use!(incremental_witness);
mod_use!(int_id);
mod_use!(join_split_description);
mod_use!(mnemonic_language);
mod_use!(network);
mod_use!(orchard_action_description);
mod_use!(orchard_sent_output);
mod_use!(parseable_types);
mod_use!(phgr_proof);
mod_use!(position);
mod_use!(sapling_incoming_viewing_key);
mod_use!(sapling_output_description);
mod_use!(sapling_sent_output);
mod_use!(sapling_spend_description);
mod_use!(sapling_witness);
mod_use!(script);
mod_use!(seconds_since_epoch);
mod_use!(seed_material);
mod_use!(shielded_address);
mod_use!(spending_key);
mod_use!(sprout_proof);
mod_use!(sprout_witness);
mod_use!(transaction);
mod_use!(transparent_address);
mod_use!(transparent_spend_authority);
mod_use!(tx_in);
mod_use!(tx_out_point);
mod_use!(tx_out);
mod_use!(txid);
mod_use!(u160_type);
mod_use!(u252_type);
mod_use!(u256_type);
mod_use!(zewif_wallet);

use std::collections::HashMap;

use bc_components::ARID;

use crate::impl_attachable;

/// Represents a Zewif wallet database, the top level of the interchange format
/// hierarchy, which can contain multiple wallets and a global transaction
/// history.
#[derive(Debug, Clone)]
pub struct ZewifTop {
    wallets: HashMap<ARID, ZewifWallet>,
    transactions: HashMap<TxId, Transaction>,
    attachments: Attachments,
}

impl_attachable!(ZewifTop);

impl ZewifTop {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            transactions: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn wallets(&self) -> &HashMap<ARID, ZewifWallet> {
        &self.wallets
    }

    pub fn add_wallet(&mut self, wallet: ZewifWallet) {
        self.wallets.insert(wallet.id().clone(), wallet);
    }

    pub fn transactions(&self) -> &HashMap<TxId, Transaction> {
        &self.transactions
    }

    pub fn transactions_mut(&mut self) -> &mut HashMap<TxId, Transaction> {
        &mut self.transactions
    }

    pub fn add_transaction(&mut self, txid: TxId, transaction: Transaction) {
        self.transactions.insert(txid, transaction);
    }

    pub fn get_transaction(&self, txid: TxId) -> Option<&Transaction> {
        self.transactions.get(&txid)
    }

    pub fn set_transactions(&mut self, transactions: HashMap<TxId, Transaction>) {
        self.transactions = transactions;
    }
}

impl Default for ZewifTop {
    fn default() -> Self {
        Self::new()
    }
}
