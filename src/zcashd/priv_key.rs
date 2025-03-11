use anyhow::{bail, Result};

use crate::{parse, u256, CompactSize, Data, Parse, Parser};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PrivKey {
    pub data: Data,
    pub hash: u256,
}

impl std::fmt::Debug for PrivKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PrivKey({:?})", &self.data)
    }
}

impl AsRef<Data> for PrivKey {
    fn as_ref(&self) -> &Data {
        &self.data
    }
}

impl AsRef<[u8]> for PrivKey {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Parse for PrivKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let length = *parse!(p, CompactSize, "PrivKey size")?;
        if length != 214 && length != 279 {
            bail!("Invalid PrivKey size: {}", length);
        }
        let data = parse!(p, data = length, "PrivKey")?;
        let hash = parse!(p, "PrivKey hash")?;
        Ok(Self { data, hash })
    }
}
