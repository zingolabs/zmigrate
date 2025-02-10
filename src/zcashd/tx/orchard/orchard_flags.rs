use anyhow::Result;

use crate::{parse, Parse, Parser};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrchardFlags {
    spends_enabled: bool,
    outputs_enabled: bool,
}

impl Parse for OrchardFlags  {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            spends_enabled: parse!(parser, "OrchardFlags.spends_enabled")?,
            outputs_enabled: parse!(parser, "OrchardFlags.outputs_enabled")?,
        })
    }
}
