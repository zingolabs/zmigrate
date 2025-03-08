use anyhow::Result;

use crate::{blob, data, parse, Parse, Parser};

use super::{Versioned, ChainCode};

blob!(PublicKey, 33);

#[derive(Debug, Clone)]
pub struct LegacyExtendedPubKey {
    pub version: u8,
    pub public_key: PublicKey,
    pub chain_code: ChainCode,
}

impl Versioned for LegacyExtendedPubKey {
    const VERSION: u8 = 1;
}

impl Parse for LegacyExtendedPubKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version = Self::get_version(p)?;
        let public_key = parse!(p, "public_key")?;
        let chain_code = parse!(p, "chain_code")?;
        Ok(Self { version, public_key, chain_code })
    }
}
