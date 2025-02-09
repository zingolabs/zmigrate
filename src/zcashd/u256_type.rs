use anyhow::{ Result, Context };

use crate::{ parse, Blob32, Parse, Parser };

#[derive(Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u256(Blob32);

impl u256 {
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

impl AsRef<Blob32> for u256 {
    fn as_ref(&self) -> &Blob32 {
        &self.0
    }
}

impl AsRef<[u8]> for u256 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Debug for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "U256({})", hex::encode(self.as_blob()))
    }
}

impl Parse for u256 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, "u256")?;
        Ok(Self(blob))
    }
}
