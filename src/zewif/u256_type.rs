use anyhow::{Context, Result};

use crate::{Blob32, Parse, Parser, parse};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u256([u8; 32]);

impl u256 {
    pub fn from_blob(blob: Blob32) -> Self {
        Self(blob.into())
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let blob = Blob32::from_slice(bytes).context("Creating U256 from slice")?;
        Ok(Self(blob.into()))
    }

    pub fn from_hex(hex: &str) -> Self {
        let blob = Blob32::from_hex(hex);
        Self(blob.into())
    }
}

impl AsRef<[u8]> for u256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for u256 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u256({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

impl Parse for u256 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let bytes = parse!(p, "u256")?;
        Ok(Self(bytes))
    }
}
