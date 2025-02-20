use std::collections::HashMap;

use super::{
    u256, Address, BlockLocator, ClientVersion, KeyPoolEntry, Keys, MnemonicHDChain, MnemonicSeed, NetworkInfo, OrchardNoteCommitmentTree, PubKey, SproutKeys, WalletTx
};

pub struct ZcashdWallet {
    pub bestblock_nomerkle: Option<BlockLocator>,
    pub bestblock: BlockLocator,
    pub client_version: ClientVersion,
    pub default_key: PubKey,
    pub keys: Keys,
    pub sprout_keys: Option<SproutKeys>,
    pub min_version: ClientVersion,
    pub mnemonic_hd_chain: MnemonicHDChain,
    pub mnemonic_phrase: MnemonicSeed,
    pub address_names: HashMap<Address, String>,
    pub address_purposes: HashMap<Address, String>,
    pub network_info: NetworkInfo,
    pub orchard_note_commitment_tree: OrchardNoteCommitmentTree,
    pub orderposnext: Option<i64>,
    pub witnesscachesize: i64,
    pub key_pool: HashMap<i64, KeyPoolEntry>,
    pub transactions: HashMap<u256, WalletTx>,
}

impl std::fmt::Debug for ZcashdWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ZcashdWallet")
            .field("client_version", &self.client_version)
            .field("min_version", &self.min_version)
            .field("default_key", &self.default_key)
            .field("keys", &self.keys)
            .field("sprout_keys", &self.sprout_keys)
            .field("bestblock", &self.bestblock)
            .field("bestblock_nomerkle", &self.bestblock_nomerkle)
            .field("mnemonic_hd_chain", &self.mnemonic_hd_chain)
            .field("mnemonic_phrase", &self.mnemonic_phrase)
            .field("address_names", &self.address_names)
            .field("address_purposes", &self.address_purposes)
            .field("network_info", &self.network_info)
            .field("orchard_note_commitment_tree", &self.orchard_note_commitment_tree)
            .field("orderposnext", &self.orderposnext)
            .field("witnesscachesize", &self.witnesscachesize)
            .field("key_pool", &self.key_pool)
            .field("transactions", &self.transactions)
            .finish()
    }
}
