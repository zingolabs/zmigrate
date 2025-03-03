use anyhow::Result;

use crate::{parse, u160, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyId(pub u160);

impl Parse for KeyId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let key_id = parse!(p, "key_id")?;
        Ok(KeyId(key_id))
    }
}
