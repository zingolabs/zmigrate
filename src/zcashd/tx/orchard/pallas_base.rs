use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::Fp;

#[derive(Debug, Clone, PartialEq)]
pub struct PallasBase(Fp);

impl Parse for PallasBase {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(parser, "PallasBase")?))
    }
}
