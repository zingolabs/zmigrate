use anyhow::{ Result, Context };

use crate::{ Blob32, Parseable };

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct U256(Blob32);

impl U256 {
    pub fn from_blob(blob: Blob32) -> Self {
        Self(blob)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let blob = Blob32::from_slice(bytes).context("Creating U256 from slice")?;
        Ok(Self(blob))
    }

    pub fn as_blob(&self) -> &Blob32 {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Debug for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "U256({})", hex::encode(self.as_blob()))
    }
}

impl Parseable for U256 {
    fn parse_type() -> &'static str {
        "u256"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let blob = Blob32::parse(parser).context("Parsing u256")?;
        Ok(Self(blob))
    }
}
