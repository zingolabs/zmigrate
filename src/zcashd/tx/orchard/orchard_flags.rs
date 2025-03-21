use anyhow::Result;

use crate::{Parse, Parser, parse};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrchardFlags {
    spends_enabled: bool,
    outputs_enabled: bool,
}

const FLAG_SPENDS_ENABLED: u8 = 0b0000_0001;
const FLAG_OUTPUTS_ENABLED: u8 = 0b0000_0010;

impl Parse for OrchardFlags {
    fn parse(parser: &mut Parser) -> Result<Self>
    where
        Self: Sized,
    {
        let value = parse!(parser, u8, "OrchardFlags")?;
        let spends_enabled = (value & FLAG_SPENDS_ENABLED) != 0;
        let outputs_enabled = (value & FLAG_OUTPUTS_ENABLED) != 0;
        Ok(Self { spends_enabled, outputs_enabled })
    }
}
