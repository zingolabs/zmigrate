use std::collections::HashMap;

use super::{SaplingIncomingViewingKey, SaplingKey};

#[derive(Clone, PartialEq)]
pub struct SaplingKeys (pub HashMap<SaplingIncomingViewingKey, SaplingKey>);

impl SaplingKeys {
    pub fn new(map: HashMap<SaplingIncomingViewingKey, SaplingKey>) -> Self {
        Self(map)
    }

    pub fn get(&self, key: &SaplingIncomingViewingKey) -> Option<&SaplingKey> {
        self.0.get(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn keypairs(&self) -> impl Iterator<Item = &SaplingKey> {
        self.0.values()
    }
}

impl std::fmt::Debug for SaplingKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut a = f.debug_list();
        for keypair in self.keypairs() {
            a.entry(keypair);
        }
        a.finish()
    }
}
