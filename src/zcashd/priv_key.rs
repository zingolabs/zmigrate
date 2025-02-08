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
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let length = parse_compact_size(parser).context("Parsing PrivKey size")?;
        if length != 214 && length != 279 {
            bail!("Invalid PrivKey size: {}", length);
        }
        let data = parse!(data length, parser, "Parsing PrivKey")?;
        let hash = parse!(parser, "Parsing PrivKey hash")?;
        Ok(Self { data, hash })
    }
}
