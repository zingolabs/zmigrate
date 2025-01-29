use crate::Parseable;

use anyhow::Result;
use chrono::{ TimeZone, Utc, SecondsFormat };

/// Represents a number of seconds since the Unix epoch.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecondsSinceEpoch(u64);

impl SecondsSinceEpoch {
    pub fn new(seconds: u64) -> Self {
        Self(seconds)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Parseable for SecondsSinceEpoch {
    fn parse_type() -> &'static str {
        "SecondsSinceEpoch"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let seconds = parser.parse_u64()?;
        Ok(Self::new(seconds))
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
