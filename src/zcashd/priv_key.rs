use anyhow::{ Result, Context, bail };

use crate::{ U256, Data, Parseable, Parser };

use super::parse_compact_size;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PrivKey {
    data: Data,
    hash: U256,
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

    pub fn hash(&self) -> &U256 {
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

impl Parseable for PrivKey {
    fn parse_type() -> &'static str {
        "PrivKey"
    }

    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let size = parse_compact_size(parser).context("Parsing PrivKey size")?;
        if size != 214 && size != 279 {
            bail!("Invalid PrivKey size: {}", size);
        }
        let data = Data::parse(size, parser).context("Parsing PrivKey")?;
        let hash = U256::parse(parser).context("Parsing PrivKey hash")?;
        Ok(Self { data, hash })
    }
}
