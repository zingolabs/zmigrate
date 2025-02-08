use anyhow::{ Result, Context };

use crate::{ Blob20, Parse, Parser };

#[derive(Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct u160(Blob20);

impl u160 {
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

impl AsRef<Blob20> for u160 {
    fn as_ref(&self) -> &Blob20 {
        &self.0
    }
}

impl AsRef<[u8]> for u160 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Debug for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "U160({})", hex::encode(self.as_blob()))
    }
}

impl Parse for u160 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let blob = Parse::parse(parser).context("u160")?;
        Ok(Self(blob))
    }
}
