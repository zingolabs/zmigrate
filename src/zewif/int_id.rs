use anyhow::Result;

use crate::{Parse, Parser, parse};

/// A wrapper around a 32-bit unsigned integer, displayed as a hex string.
/// Used primarily for transaction version group IDs and other protocol identifiers.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IntID(pub u32);

impl std::fmt::Display for IntID {
    // Always display as hex with `0x` prefix
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

impl std::fmt::Debug for IntID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Parse for IntID {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "IntID")?))
    }
}
