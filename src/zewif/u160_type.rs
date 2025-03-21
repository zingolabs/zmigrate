use anyhow::{Context, Result};

use crate::{Blob20, Parse, Parser, parse};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u160([u8; 20]);

impl u160 {
    pub fn from_blob(blob: Blob20) -> Self {
        Self(blob.0)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let blob = Blob20::from_slice(bytes).context("Creating U160 from slice")?;
        Ok(Self(blob.0))
    }
}

impl AsRef<[u8]> for u160 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 20]> for u160 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl std::fmt::Debug for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u160({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

impl Parse for u160 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, "u160")?;
        Ok(Self(blob))
    }
}
