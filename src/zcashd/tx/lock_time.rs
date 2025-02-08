use anyhow::Result;

use crate::{parse, Parse, Parser, SecondsSinceEpoch};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LockTime {
    BlockHeight(u32),
    Timestamp(SecondsSinceEpoch),
}

impl LockTime {
    pub fn from_u32(locktime: u32) -> Self {
        if locktime < 500_000_000 {
            LockTime::BlockHeight(locktime)
        } else {
            LockTime::Timestamp(locktime.into())
        }
    }

    pub fn is_block_height(&self) -> bool {
        matches!(self, LockTime::BlockHeight(_))
    }

    pub fn is_timestamp(&self) -> bool {
        matches!(self, LockTime::Timestamp(_))
    }

    pub fn as_block_height(&self) -> Option<u32> {
        match self {
            LockTime::BlockHeight(height) => Some(*height),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<SecondsSinceEpoch> {
        match self {
            LockTime::Timestamp(ts) => Some(*ts),
            _ => None,
        }
    }
}

impl Default for LockTime {
    fn default() -> Self {
        LockTime::BlockHeight(0)
    }
}

impl Parse for LockTime {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let locktime = parse!(parser, "locktime")?;
        Ok(LockTime::from_u32(locktime))
    }
}
