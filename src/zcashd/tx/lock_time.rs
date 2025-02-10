use anyhow::Result;

use crate::{parse, Parse, Parser, SecondsSinceEpoch};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LockTime {
    BlockHeight(u32),
    Timestamp(SecondsSinceEpoch),
}

impl LockTime {
    pub fn from_u32(locktime: u32) -> Self {
        if locktime < 500_000_000 {
            Self::BlockHeight(locktime)
        } else {
            Self::Timestamp(locktime.into())
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

    pub fn as_option(self) -> Option<Self> {
        match self {
            LockTime::BlockHeight(0) => None,
            _ => Some(self),
        }
    }
}

impl Parse for LockTime {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(LockTime::from_u32(parse!(p, "locktime")?))
    }
}
