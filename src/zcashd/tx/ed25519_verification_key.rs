use anyhow::Result;

use crate::{ parse, Blob32, Parse, Parser };

#[derive(Clone, PartialEq)]
pub struct Ed25519VerificationKey(Blob32);

impl AsRef<Blob32> for Ed25519VerificationKey {
    fn as_ref(&self) -> &Blob32 {
        &self.0
    }
}

impl AsRef<[u8]> for Ed25519VerificationKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Parse for Ed25519VerificationKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, "Ed25519VerificationKey")?;
        Ok(Self(blob))
    }
}

impl std::fmt::Debug for Ed25519VerificationKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ed25519VerificationKey({})", hex::encode(self))
    }
}
