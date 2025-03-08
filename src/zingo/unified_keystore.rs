use anyhow::{anyhow, bail, Context, Result};

use crate::{parse, sapling::SaplingExtendedSpendingKey, Parse};

use super::{
    Era, LegacyExtendedPrivKey, OrchardSpendingKey, UnifiedFullViewingKey, UnifiedSpendingKey,
    Versioned, KEY_TYPE_EMPTY, KEY_TYPE_SPEND, KEY_TYPE_VIEW,
};

#[derive(Debug, Clone)]
pub enum UnifiedKeystore {
    Spend(Box<UnifiedSpendingKey>),
    View(Box<UnifiedFullViewingKey>),
    Empty,
}

impl Versioned for UnifiedKeystore {
    const VERSION: u8 = 0;
}

impl Parse for UnifiedKeystore {
    fn parse(p: &mut crate::Parser) -> Result<Self> {
        let _version = Self::get_version(p)?;
        let key_type = parse!(p, u8, "key_type")?;
        Ok(match key_type {
            KEY_TYPE_SPEND => UnifiedKeystore::Spend(Box::new(
                UnifiedSpendingKey::parse_with_size(p).with_context(|| "spending")?,
            )),
            KEY_TYPE_VIEW => UnifiedKeystore::View(Box::new(parse!(p, "view")?)),
            KEY_TYPE_EMPTY => UnifiedKeystore::Empty,
            _ => bail!("Unknown wallet UnifiedKeystore type: {}", key_type),
        })
    }
}
