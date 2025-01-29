use anyhow::Result;

use crate::{Blob32, Parseable, SecondsSinceEpoch};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMetadata {
    version: i32,
    create_time: SecondsSinceEpoch,
    hd_keypath: String,
    seed_fp: Blob32,
}

impl KeyMetadata {
    pub fn new(version: i32, create_time: SecondsSinceEpoch, hd_keypath: String, seed_fp: Blob32) -> Self {
        Self {
            version,
            create_time,
            hd_keypath,
            seed_fp,
        }
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn create_time(&self) -> &SecondsSinceEpoch {
        &self.create_time
    }

    pub fn hd_keypath(&self) -> &str {
        &self.hd_keypath
    }

    pub fn seed_fp(&self) -> &Blob32 {
        &self.seed_fp
    }
}

impl Parseable for KeyMetadata {
    fn parse_type() -> &'static str {
        "KeyMetadata"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let version = parser.parse_i32()?;
        let create_time = SecondsSinceEpoch::parse(parser)?;
        let hd_keypath = parser.parse_utf8()?;
        let seed_fp = parser.parse_blob()?;
        Ok(Self::new(version, create_time, hd_keypath, seed_fp))
    }
}
