use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::Ep;

#[derive(Debug, Clone, PartialEq)]
pub struct PallasPoint(Ep);

impl Parse for PallasPoint {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(p, "PallasPoint")?))
    }
}
