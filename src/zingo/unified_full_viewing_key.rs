use anyhow::{anyhow, bail, Result};

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone)]
pub struct UnifiedFullViewingKey(String);

impl Parse for UnifiedFullViewingKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let key = parse!(p, "UnifiedFullViewingKey")?;
        Ok(Self(key))
    }
}
