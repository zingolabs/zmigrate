use anyhow::{ Result, Context, bail };

use crate::{ Data, Parseable, Parser };

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PubKey(Data);

impl std::fmt::Debug for PubKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PubKey({:?})", &self.0)
    }
}

impl PubKey {
    pub fn as_data(&self) -> &Data {
        &self.0
    }
}

impl AsRef<Data> for PubKey {
    fn as_ref(&self) -> &Data {
        &self.0
    }
}

impl AsRef<[u8]> for PubKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Parseable for PubKey {
    fn parse_type() -> &'static str {
        "PubKey"
    }

    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let size = parser.parse_compact_size().context("Parsing PubKey size")?;
        if size != 33 && size != 65 {
            bail!("Invalid PubKey size: {}", size);
        }
        let key_data = Data::parse(size, parser).context("Parsing PubKey")?;
        Ok(Self(key_data))
    }
}
