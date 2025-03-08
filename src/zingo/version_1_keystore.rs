use crate::{parse, sapling::SaplingExtendedSpendingKey, Parse};

use super::{LegacyExtendedPrivKey, OrchardSpendingKey};

#[derive(Debug, Clone)]
pub struct Version1Keystore {
    pub orchard: OrchardSpendingKey,
    pub sapling: SaplingExtendedSpendingKey,
    pub transparent: LegacyExtendedPrivKey,
}

impl Parse for Version1Keystore {
    fn parse(p: &mut crate::Parser) -> anyhow::Result<Self> {
        let orchard = parse!(p, "orchard")?;
        let sapling = parse!(p, "sapling")?;
        let transparent = parse!(p, "transparent")?;
        Ok(Self { orchard, sapling, transparent })
    }
}
