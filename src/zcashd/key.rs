use anyhow::{Result, bail};

use crate::{hash256, Data};

use super::{KeyMetadata, PrivKey, PubKey};

#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    pub pubkey: PubKey,
    pub privkey: PrivKey,
    pub metadata: KeyMetadata,
}

impl Key {
    pub fn new(pubkey: PubKey, privkey: PrivKey, metadata: KeyMetadata) -> Result<Self> {
        let hash = hash256(Data::concat(&[&pubkey, &privkey]));
        if hash != privkey.hash {
            bail!("Invalid keypair: pubkey and privkey do not match");
        }
        Ok(Self { pubkey, privkey, metadata })
    }
}
