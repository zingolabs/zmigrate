use anyhow::Result;

use crate::{blob, data, parse, Parse, Parser};

use super::Versioned;

blob!(SecretKey, 32);
data!(ChainCode);

#[derive(Debug, Clone)]
pub struct LegacyExtendedPrivKey {
    pub version: u8,
    pub private_key: SecretKey,
    pub chain_code: ChainCode,
}

impl Versioned for LegacyExtendedPrivKey {
    const VERSION: u8 = 1;
}

impl Parse for LegacyExtendedPrivKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version = Self::get_version(p)?;
        let private_key = parse!(p, "private_key")?;
        let chain_code = parse!(p, "chain_code")?;
        Ok(Self { version, private_key, chain_code })
    }
}
