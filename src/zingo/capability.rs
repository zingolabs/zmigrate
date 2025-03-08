use anyhow::{anyhow, bail, Result};

use crate::{parse, Parse, Parser};

use super::Versioned;

pub const KEY_TYPE_EMPTY: u8 = 0;
pub const KEY_TYPE_VIEW: u8 = 1;
pub const KEY_TYPE_SPEND: u8 = 2;

#[derive(Clone, Debug)]
pub enum Capability<ViewingKeyType, SpendKeyType> {
    None,
    View(ViewingKeyType),
    Spend(SpendKeyType),
}

impl<ViewingKeyType, SpendKeyType> Versioned for Capability<ViewingKeyType, SpendKeyType> {
    const VERSION: u8 = 1;
}

impl<ViewingKeyType, SpendKeyType> Parse for Capability<ViewingKeyType, SpendKeyType>
where
    ViewingKeyType: Parse,
    SpendKeyType: Parse,
{
    fn parse(p: &mut Parser) -> Result<Self> {
        Self::get_version(p)?;
        let capability_type = parse!(p, u8, "capability_type")?;
        match capability_type {
            KEY_TYPE_EMPTY => Ok(Capability::None),
            KEY_TYPE_VIEW => Ok(Capability::View(ViewingKeyType::parse(p)?)),
            KEY_TYPE_SPEND => Ok(Capability::Spend(SpendKeyType::parse(p)?)),
            _ => bail!("Unknown wallet Capability type: {}", capability_type),
        }
    }
}
