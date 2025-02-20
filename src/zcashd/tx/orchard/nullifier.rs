use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::Fp;

#[derive(Debug, Clone, PartialEq)]
pub struct Nullifier(pub Fp);

impl Parse for Nullifier {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(p, "Nullifier")?))
    }
}
