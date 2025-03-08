use anyhow::Result;

use crate::{Parse, Parser, parse, sapling::SaplingExtendedSpendingKey};

use super::{Capability, LegacyExtendedPrivKey, LegacyExtendedPubKey, OrchardFullViewingKey, OrchardSpendingKey, SaplingDiversifiableFullViewingKey};

#[derive(Debug, Clone)]
pub struct Version2Keystore {
    pub orchard: Capability<OrchardFullViewingKey, OrchardSpendingKey>,
    pub sapling: Capability<SaplingDiversifiableFullViewingKey, SaplingExtendedSpendingKey>,
    pub transparent: Capability<LegacyExtendedPubKey, LegacyExtendedPrivKey>,
}

impl Parse for Version2Keystore {
    fn parse(p: &mut Parser) -> Result<Self> {
        let orchard = parse!(p, "orchard")?;
        let sapling = parse!(p, "sapling")?;
        let transparent = parse!(p, "transparent")?;
        Ok(Self { orchard, sapling, transparent })
    }
}
