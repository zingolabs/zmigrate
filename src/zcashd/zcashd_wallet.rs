use std::collections::HashMap;

use crate::Blob32;

use super::{Address, BlockLocator, ClientVersion, KeyPoolEntry, Keys, MnemonicHDChain, MnemonicSeed, NetworkInfo, OrchardNoteCommitmentTree, PubKey, Transaction};

pub struct ZcashdWallet {
    bestblock_nomerkle: BlockLocator,
    bestblock: BlockLocator,
    client_version: ClientVersion,
    default_key: PubKey,
    keys: Keys,
    min_version: ClientVersion,
    mnemonic_hd_chain: MnemonicHDChain,
    mnemonic_phrase: MnemonicSeed,
    address_names: HashMap<Address, String>,
    address_purposes: HashMap<Address, String>,
    network_info: NetworkInfo,
    orchard_note_commitment_tree: OrchardNoteCommitmentTree,
    orderposnext: i64,
    witnesscachesize: i64,
    key_pool: HashMap<i64, KeyPoolEntry>,
    transactions: HashMap<Blob32, Transaction>,
}

impl std::fmt::Debug for ZcashdWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ZcashdWallet")
            .field("client_version", &self.client_version)
            .field("min_version", &self.min_version)
            .field("default_key", &self.default_key)
            .field("keys", &self.keys)
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

impl ZcashdWallet {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bestblock_nomerkle: BlockLocator,
        bestblock: BlockLocator,
        client_version: ClientVersion,
        default_key: PubKey,
        keys: Keys,
        min_version: ClientVersion,
        mnemonic_hd_chain: MnemonicHDChain,
        mnemonic_phrase: MnemonicSeed,
        address_names: HashMap<Address, String>,
        address_purposes: HashMap<Address, String>,
        network_info: NetworkInfo,
        orchard_note_commitment_tree: OrchardNoteCommitmentTree,
        orderposnext: i64,
        witnesscachesize: i64,
        key_pool: HashMap<i64, KeyPoolEntry>,
        transactions: HashMap<Blob32, Transaction>,
    ) -> Self {
        Self {
            bestblock_nomerkle,
            bestblock,
            client_version,
            default_key,
            keys,
            min_version,
            mnemonic_hd_chain,
            mnemonic_phrase,
            address_names,
            address_purposes,
            network_info,
            orchard_note_commitment_tree,
            orderposnext,
            witnesscachesize,
            key_pool,
            transactions,
        }
    }

    pub fn best_block_nomerkle(&self) -> &BlockLocator {
        &self.bestblock_nomerkle
    }

    pub fn best_block(&self) -> &BlockLocator {
        &self.bestblock
    }

    pub fn client_version(&self) -> &ClientVersion {
        &self.client_version
    }

    pub fn default_key(&self) -> &PubKey {
        &self.default_key
    }

    pub fn keys(&self) -> &Keys {
        &self.keys
    }

    pub fn min_version(&self) -> &ClientVersion {
        &self.min_version
    }

    pub fn mnemonic_hd_chain(&self) -> &MnemonicHDChain {
        &self.mnemonic_hd_chain
    }

    pub fn mnemonic_phrase(&self) -> &MnemonicSeed {
        &self.mnemonic_phrase
    }

    pub fn address_names(&self) -> &HashMap<Address, String> {
        &self.address_names
    }

    pub fn network_info(&self) -> &NetworkInfo {
        &self.network_info
    }

    pub fn orchard_note_commitment_tree(&self) -> &OrchardNoteCommitmentTree {
        &self.orchard_note_commitment_tree
    }

    pub fn orderposnext(&self) -> i64 {
        self.orderposnext
    }

    pub fn witnesscachesize(&self) -> i64 {
        self.witnesscachesize
    }

    pub fn key_pool(&self) -> &HashMap<i64, KeyPoolEntry> {
        &self.key_pool
    }

    pub fn transactions(&self) -> &HashMap<Blob32, Transaction> {
        &self.transactions
    }
}
