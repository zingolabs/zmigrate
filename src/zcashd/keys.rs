use std::collections::HashMap;

use super::{Key, PubKey};

#[derive(Clone, PartialEq)]
pub struct Keys (HashMap<PubKey, Key>);

impl Keys {
    pub fn new(map: HashMap<PubKey, Key>) -> Self {
        Self(map)
    }

    pub fn map(&self) -> &HashMap<PubKey, Key> {
        &self.0
    }

    pub fn get(&self, pubkey: &PubKey) -> Option<&Key> {
        self.0.get(pubkey)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn keypairs(&self) -> impl Iterator<Item = &Key> {
        self.0.values()
    }
}

impl std::fmt::Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut a = f.debug_list();
        for keypair in self.keypairs() {
            a.entry(keypair);
        }
        a.finish()
    }
}
