use crate::{Parse, Parser};

use anyhow::Result;
use chrono::{ TimeZone, Utc, SecondsFormat };

/// Represents a number of seconds since the Unix epoch.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecondsSinceEpoch(u64);

impl SecondsSinceEpoch {
    pub fn from_u64(seconds: u64) -> Self {
        Self(seconds)
    }

    pub fn from_u32(seconds: u32) -> Self {
        Self(seconds as u64)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for SecondsSinceEpoch {
    fn from(seconds: u64) -> Self {
        Self::from_u64(seconds)
    }
}

impl From<u32> for SecondsSinceEpoch {
    fn from(seconds: u32) -> Self {
        Self::from_u32(seconds)
    }
}

impl Parse for SecondsSinceEpoch {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let seconds = u64::parse(parser)?;
        Ok(Self::from_u64(seconds))
    }
}

// Format as ISO-8601 date-time, e.g.: "2024-11-27T09:39:36Z"
impl std::fmt::Debug for SecondsSinceEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dt = Utc.timestamp_opt(self.0 as i64, 0)
            .single()
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Secs, true);
        write!(f, "{}", dt)
    }
}
