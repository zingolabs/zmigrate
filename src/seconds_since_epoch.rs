use crate::Parseable;

use anyhow::Result;

/// Represents a number of seconds since the Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
