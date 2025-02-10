use anyhow::Result;

use super::PallasPoint;

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct ValueCommitment(PallasPoint);

impl Parse for ValueCommitment {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(p, "ValueCommitment")?))
    }
}
