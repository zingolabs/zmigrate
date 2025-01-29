use super::Keys;

pub struct ZcashdWallet {
    version: i32,
    keys: Keys,
}

impl std::fmt::Debug for ZcashdWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut a = f.debug_struct("ZcashdWallet");
        a.field("version", &self.version);
        a.field("keys", &self.keys);
        a.finish()
    }
}

impl ZcashdWallet {
    pub fn new(version: i32, keys: Keys) -> Self {
        Self {
            version,
            keys,
        }
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn keys(&self) -> &Keys {
        &self.keys
    }
}
