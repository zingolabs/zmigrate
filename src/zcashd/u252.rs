use anyhow::{ Result, Context, bail };

use crate::{ Blob32, Parseable };

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct U252(Blob32);

impl U252 {
    pub fn from_blob(blob: Blob32) -> Result<Self> {
        if (blob.as_bytes()[0] & 0xf0) != 0 {
            bail!("First four bits of u252 must be zero");
        }
        Ok(Self(blob))
    }

    pub fn as_blob(&self) -> &Blob32 {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Debug for U252 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "U252({})", hex::encode(self.as_blob()))
    }
}

impl Parseable for U252 {
    fn parse_type() -> &'static str {
        "u252"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let blob = Blob32::parse(parser).context("Parsing u252")?;
        Self::from_blob(blob)
    }
}
