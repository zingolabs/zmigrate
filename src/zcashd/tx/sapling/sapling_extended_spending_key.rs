use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

use super::SaplingExpandedSpendingKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingExtendedSpendingKey {
    pub depth: u8,
    pub parent_fvk_tag: u32,
    pub child_index: u32,
    pub chain_code: u256,
    pub expsk: SaplingExpandedSpendingKey,
    pub dk: u256,
}

impl Parse for SaplingExtendedSpendingKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(SaplingExtendedSpendingKey {
            depth: parse!(p, "depth")?,
            parent_fvk_tag: parse!(p, "parent_fvk_tag")?,
            child_index: parse!(p, "child_index")?,
            chain_code: parse!(p, "chain_code")?,
            expsk: parse!(p, "expsk")?,
            dk: parse!(p, "dk")?,
        })
    }
}
