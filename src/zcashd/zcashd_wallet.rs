use super::{BlockLocator, ClientVersion, Keys, MnemonicHDChain, MnemonicSeed, PubKey};

pub struct ZcashdWallet {
    bestblock_nomerkle: BlockLocator,
    bestblock: BlockLocator,
    client_version: ClientVersion,
    default_key: PubKey,
    keys: Keys,
    min_version: ClientVersion,
    mnemonic_hd_chain: MnemonicHDChain,
    mnemonic_phrase: MnemonicSeed,
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
}
