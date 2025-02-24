use std::collections::HashMap;

use super::{
    u256, Address, BlockLocator, ClientVersion, KeyPoolEntry, Keys, MnemonicHDChain, MnemonicSeed,
    NetworkInfo, OrchardNoteCommitmentTree, PubKey, SaplingIncomingViewingKey, SaplingKeys,
    SaplingZPaymentAddress, SproutKeys, UnifiedAccounts, WalletTx,
};

pub struct ZcashdWallet {
    pub address_names: HashMap<Address, String>,
    pub address_purposes: HashMap<Address, String>,
    pub bestblock_nomerkle: Option<BlockLocator>,
    pub bestblock: BlockLocator,
    pub client_version: ClientVersion,
    pub default_key: PubKey,
    pub key_pool: HashMap<i64, KeyPoolEntry>,
    pub keys: Keys,
    pub min_version: ClientVersion,
    pub mnemonic_hd_chain: MnemonicHDChain,
    pub mnemonic_phrase: MnemonicSeed,
    pub network_info: NetworkInfo,
    pub orchard_note_commitment_tree: OrchardNoteCommitmentTree,
    pub orderposnext: Option<i64>,
    pub sapling_keys: SaplingKeys,
    pub sapling_z_addresses: HashMap<SaplingZPaymentAddress, SaplingIncomingViewingKey>,
    pub sprout_keys: Option<SproutKeys>,
    pub transactions: HashMap<u256, WalletTx>,
    pub unified_accounts: Option<UnifiedAccounts>,
    pub witnesscachesize: i64,
}

impl std::fmt::Debug for ZcashdWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ZcashdWallet")
            .field("address_names", &self.address_names)
            .field("address_purposes", &self.address_purposes)
            .field("bestblock_nomerkle", &self.bestblock_nomerkle)
            .field("bestblock", &self.bestblock)
            .field("client_version", &self.client_version)
            .field("default_key", &self.default_key)
            .field("key_pool", &self.key_pool)
            .field("keys", &self.keys)
            .field("min_version", &self.min_version)
            .field("mnemonic_hd_chain", &self.mnemonic_hd_chain)
            .field("mnemonic_phrase", &self.mnemonic_phrase)
            .field("network_info", &self.network_info)
            .field(
                "orchard_note_commitment_tree",
                &self.orchard_note_commitment_tree,
            )
            .field("orderposnext", &self.orderposnext)
            .field("sapling_keys", &self.sapling_keys)
            .field("sapling_z_addresses", &self.sapling_z_addresses)
            .field("sprout_keys", &self.sprout_keys)
            .field("transactions", &self.transactions)
            .field("unified_accounts", &self.unified_accounts)
            .field("witnesscachesize", &self.witnesscachesize)
            .finish()
    }
}
