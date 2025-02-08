use anyhow::Result;
use crate::{Parse, Parser};

use super::parse_pair;

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfo {
    zcash: String,
    identifier: String,
}

impl NetworkInfo {
    pub fn zcash(&self) -> &str {
        &self.zcash
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl Parse for NetworkInfo {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let (zcash, identifier) = parse_pair(parser)?;
        Ok(Self { zcash, identifier })
    }
}
