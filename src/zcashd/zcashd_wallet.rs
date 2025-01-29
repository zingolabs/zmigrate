use super::{BlockLocator, ClientVersion, Keys, PubKey};

pub struct ZcashdWallet {
    client_version: ClientVersion,
    min_version: ClientVersion,
    keys: Keys,
    default_key: PubKey,
    bestblock: BlockLocator,
    bestblock_nomerkle: BlockLocator,
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
            .finish()
    }
}

impl ZcashdWallet {
    pub fn new(
        client_version: ClientVersion,
        min_version: ClientVersion,
        default_key: PubKey,
        keys: Keys,
        bestblock: BlockLocator,
        bestblock_nomerkle: BlockLocator,
    ) -> Self {
        Self {
            client_version,
            min_version,
            default_key,
            keys,
            bestblock,
            bestblock_nomerkle,
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
