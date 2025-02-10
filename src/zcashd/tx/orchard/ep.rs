use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::Fp;

#[derive(Debug, Clone, PartialEq)]
pub struct Ep {
    x: Fp,
    y: Fp,
    z: Fp,
}

impl Parse for Ep {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            x: parse!(p, "x")?,
            y: parse!(p, "y")?,
            z: parse!(p, "z")?,
        })
    }
}
