use anyhow::{Error, Result, anyhow};
use zcash_address::{ToAddress, TryFromAddress, ZcashAddress};

use crate::{Blob, Network, Parse, Parser, parse};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingZPaymentAddress {
    diversifier: Blob<11>,
    pk: Blob<32>,
}

impl SaplingZPaymentAddress {
    pub fn to_string(&self, network: Network) -> String {
        // Concatenate diversifier (11 bytes) and pk (32 bytes) into a 43-byte array
        let mut bytes = [0u8; 43];
        bytes[..11].copy_from_slice(&self.diversifier.0);
        bytes[11..].copy_from_slice(&self.pk.0);
        let addr = ZcashAddress::from_sapling(network, bytes);
        addr.to_string()
    }

    pub fn diversifier(&self) -> &Blob<11> {
        &self.diversifier
    }

    pub fn pk(&self) -> &Blob<32> {
        &self.pk
    }
}

impl Parse for SaplingZPaymentAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        let diversifier = parse!(p, "diversifier")?;
        let pk = parse!(p, "pk")?;
        Ok(SaplingZPaymentAddress { diversifier, pk })
    }
}
