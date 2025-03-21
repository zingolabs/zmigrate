use anyhow::Result;

use crate::{Parse, Parser, parse, u256};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnifiedAccountMetadata {
    pub seed_fingerprint: u256,
    pub bip_44_coin_type: u32,
    pub account_id: u32,
    pub key_id: u256,
}

impl Parse for UnifiedAccountMetadata {
    fn parse(p: &mut Parser) -> Result<Self> {
        let seed_fingerprint = parse!(p, "seed_fingerprint")?;
        let bip_44_coin_type = parse!(p, "bip_44_coin_type")?;
        let account_id = parse!(p, "account_id")?;
        let key_id = parse!(p, "key_id")?;
        Ok(Self {
            seed_fingerprint,
            bip_44_coin_type,
            account_id,
            key_id,
        })
    }
}
