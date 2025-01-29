use super::{BlockLocator, ClientVersion, Keys, MnemonicHDChain, PubKey};

pub struct ZcashdWallet {
    bestblock_nomerkle: BlockLocator,
    bestblock: BlockLocator,
    client_version: ClientVersion,
    default_key: PubKey,
    keys: Keys,
    min_version: ClientVersion,
    mnemonic_hd_chain: MnemonicHDChain,
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
            .finish()
    }
}

impl ZcashdWallet {
    pub fn new(
        bestblock_nomerkle: BlockLocator,
        bestblock: BlockLocator,
        client_version: ClientVersion,
        default_key: PubKey,
        keys: Keys,
        min_version: ClientVersion,
        mnemonic_hd_chain: MnemonicHDChain,
    ) -> Self {
        Self {
            bestblock_nomerkle,
            bestblock,
            client_version,
            default_key,
            keys,
            min_version,
            mnemonic_hd_chain,
        }
    }

    pub fn client_version(&self) -> &ClientVersion {
        &self.client_version
    }

    pub fn min_version(&self) -> &ClientVersion {
        &self.min_version
    }

    pub fn keys(&self) -> &Keys {
        &self.keys
    }

    pub fn best_block(&self) -> &BlockLocator {
        &self.bestblock
    }

    pub fn best_block_nomerkle(&self) -> &BlockLocator {
        &self.bestblock_nomerkle
    }
}
