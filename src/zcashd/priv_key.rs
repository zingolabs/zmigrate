use anyhow::{ Result, Context, bail };

use crate::{ parse, u256, Data, Parse, Parser };

use super::parse_compact_size;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PrivKey {
    data: Data,
    hash: u256,
}

impl std::fmt::Debug for PrivKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PrivKey({:?})", &self.data)
    }
}

impl PrivKey {
    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn hash(&self) -> &u256 {
        &self.hash
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
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        let length = parse_compact_size(p).context("PrivKey size")?;
        if length != 214 && length != 279 {
            bail!("Invalid PrivKey size: {}", length);
        }
        let data = parse!(p, data length, "PrivKey")?;
        let hash = parse!(p, "PrivKey hash")?;
        Ok(Self { data, hash })
    }
}
