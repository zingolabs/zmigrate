use anyhow::Result;
use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfo {
    pub zcash: String,
    pub identifier: String,
}

impl Parse for NetworkInfo {
    fn parse(p: &mut Parser) -> Result<Self> {
        let (zcash, identifier) = parse!(p, "(zcash, identifier)")?;
        Ok(Self { zcash, identifier })
    }
}
