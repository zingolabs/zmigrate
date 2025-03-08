use anyhow::Result;

use crate::{parse, Parse};

use super::Versioned;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ReceiverSelection {
    pub version: u8,
    pub orchard: bool,
    pub sapling: bool,
    pub transparent: bool,
}

impl Versioned for ReceiverSelection {
    const VERSION: u8 = 1;
}

impl Parse for ReceiverSelection {
    fn parse(p: &mut crate::Parser) -> Result<Self> {
        let version = Self::get_version(p)?;
        let receivers = parse!(p, u8, "receivers")?;
        Ok(Self {
            version,
            orchard: receivers & 0b1 != 0,
            sapling: receivers & 0b10 != 0,
            transparent: receivers & 0b100 != 0,
        })
    }
}
