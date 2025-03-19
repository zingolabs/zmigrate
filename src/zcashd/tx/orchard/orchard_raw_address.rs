use anyhow::Result;

use zcash_address::{unified::Encoding, ToAddress, TryFromAddress, ZcashAddress};

use crate::{parse, Blob, Blob32, Parse, Parser, Network};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrchardRawAddress {
    pub diversifier: Blob<11>,
    pub pk_d: Blob32,
}

impl OrchardRawAddress {
    pub fn to_string(&self, network: Network) -> String {
        // Concatenate diversifier (11 bytes) and pk_d (32 bytes) into a 43-byte array
        let mut bytes = [0u8; 43];
        bytes[..11].copy_from_slice(&self.diversifier.0);
        bytes[11..].copy_from_slice(&self.pk_d.0);

        // Create an Orchard receiver
        let orchard_receiver = zcash_address::unified::Receiver::Orchard(bytes);

        // Create a Unified Address with just this receiver
        let unified_addr = zcash_address::unified::Address::try_from_items(
            vec![orchard_receiver]
        ).expect("A single valid receiver should create a valid unified address");

        // Create a ZcashAddress from the unified address
        let addr = ZcashAddress::from_unified(network, unified_addr);
        addr.to_string()
    }
}

impl Parse for OrchardRawAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        let diversifier = parse!(p, "diversifier")?;
        let pk_d = parse!(p, "pk_d")?;
        Ok(OrchardRawAddress { diversifier, pk_d })
    }
}
