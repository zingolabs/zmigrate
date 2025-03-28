use anyhow::Result;
use zcash_address::{ToAddress, ZcashAddress};

use crate::{Network, Parse, Parser, parse, u160};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyId(pub u160);

impl KeyId {
    pub fn to_string(self, network: Network) -> String {
        // Create proper 20-byte array for the pubkey hash
        let mut pubkey_hash = [0u8; 20];
        pubkey_hash.copy_from_slice(self.0.as_ref());

        // Create a transparent P2PKH address using the proper constructor
        let addr = ZcashAddress::from_transparent_p2pkh(network, pubkey_hash);
        addr.to_string()
    }
}

impl Parse for KeyId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let key_id = parse!(p, "key_id")?;
        Ok(KeyId(key_id))
    }
}
