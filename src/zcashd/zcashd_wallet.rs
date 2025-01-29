use super::{ClientVersion, Keys};

pub struct ZcashdWallet {
    client_version: ClientVersion,
    keys: Keys,
}

impl std::fmt::Debug for ZcashdWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut a = f.debug_struct("ZcashdWallet");
        a.field("client_version", &self.client_version);
        a.field("keys", &self.keys);
        a.finish()
    }
}

impl ZcashdWallet {
    pub fn new(client_version: ClientVersion, keys: Keys) -> Self {
        Self {
            client_version,
            keys,
        }
    }

    pub fn client_version(&self) -> &ClientVersion {
        &self.client_version
    }

    pub fn keys(&self) -> &Keys {
        &self.keys
    }
}
