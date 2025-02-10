use anyhow::Result;

use crate::{ parse, Blob64, Parse, Parser };

#[derive(Clone, PartialEq)]
pub struct Ed25519Signature(pub Blob64);

impl AsRef<Blob64> for Ed25519Signature {
    fn as_ref(&self) -> &Blob64 {
        &self.0
    }
}

impl AsRef<[u8]> for Ed25519Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Parse for Ed25519Signature {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, "Ed25519Signature")?;
        Ok(Self(blob))
    }
}

impl std::fmt::Debug for Ed25519Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ed25519Signature({})", hex::encode(self.0.as_ref()))
    }
}
