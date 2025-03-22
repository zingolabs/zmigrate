use anyhow::Result;

use crate::{Parse, Parser, SecondsSinceEpoch, parse};

/// Represents a transaction lock time.
/// 
/// Can be either a block height or a timestamp.
/// Values below 500,000,000 are interpreted as block heights,
/// values above are interpreted as timestamps.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LockTime {
    /// A block height that must be reached before the transaction can be mined
    BlockHeight(u32),
    /// A timestamp that must be reached before the transaction can be mined
    Timestamp(SecondsSinceEpoch),
}

impl LockTime {
    /// Creates a new LockTime from a u32 value.
    /// 
    /// Values below 500,000,000 are interpreted as block heights,
    /// values above are interpreted as timestamps.
    pub fn from_u32(locktime: u32) -> Self {
        if locktime < 500_000_000 {
            Self::BlockHeight(locktime)
        } else {
            Self::Timestamp(locktime.into())
        }
    }

    /// Returns true if this is a block height lock time.
    pub fn is_block_height(&self) -> bool {
        matches!(self, LockTime::BlockHeight(_))
    }

    /// Returns true if this is a timestamp lock time.
    pub fn is_timestamp(&self) -> bool {
        matches!(self, LockTime::Timestamp(_))
    }

    /// Returns the block height if this is a block height lock time.
    pub fn as_block_height(&self) -> Option<u32> {
        match self {
            LockTime::BlockHeight(height) => Some(*height),
            _ => None,
        }
    }

    /// Returns the timestamp if this is a timestamp lock time.
    pub fn as_timestamp(&self) -> Option<SecondsSinceEpoch> {
        match self {
            LockTime::Timestamp(ts) => Some(*ts),
            _ => None,
        }
    }

    /// Returns None if this is BlockHeight(0), otherwise returns Some(self).
    /// 
    /// This is useful because BlockHeight(0) is often used to indicate "no lock time".
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