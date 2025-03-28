use anyhow::{Error, Result, anyhow};
use zcash_address::{ToAddress, TryFromAddress, ZcashAddress};

use crate::{Blob, Network, Parse, Parser, parse};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingZPaymentAddress {
    pub diversifier: Blob<11>,
    pub pk: Blob<32>,
}

impl SaplingZPaymentAddress {
    pub fn to_string(&self, network: Network) -> String {
        // Concatenate diversifier (11 bytes) and pk (32 bytes) into a 43-byte array
        let mut bytes = [0u8; 43];
        bytes[..11].copy_from_slice(self.diversifier.as_ref());
        bytes[11..].copy_from_slice(self.pk.as_ref());
        let addr = ZcashAddress::from_sapling(network, bytes);
        addr.to_string()
    }
}

impl Parse for SaplingZPaymentAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        let diversifier = parse!(p, "diversifier")?;
        let pk = parse!(p, "pk")?;
        Ok(SaplingZPaymentAddress { diversifier, pk })
    }
}
