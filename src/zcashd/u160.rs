use anyhow::{ Result, Context };

use crate::{ Blob20, Parseable };

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct U160(Blob20);

impl U160 {
    pub fn from_blob(blob: Blob20) -> Self {
        Self(blob)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let blob = Blob20::from_slice(bytes).context("Creating U160 from slice")?;
        Ok(Self(blob))
    }
    
    pub fn as_blob(&self) -> &Blob20 {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Debug for U160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "U160({})", hex::encode(self.as_blob()))
    }
}

impl Parseable for U160 {
    fn parse_type() -> &'static str {
        "u160"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let blob = Blob20::parse(parser).context("Parsing u160")?;
        Ok(Self(blob))
    }
}
