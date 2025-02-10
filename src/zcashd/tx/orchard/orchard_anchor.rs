use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::PallasBase;

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAnchor(PallasBase);

impl Parse for OrchardAnchor {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(parser, "OrchardAnchor")?))
    }
}
