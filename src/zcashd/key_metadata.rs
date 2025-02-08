use anyhow::Result;

use crate::{ Blob, Blob32, Parseable, Parser, SecondsSinceEpoch };

const VERSION_WITH_HDDATA: i32 = 10;
#[derive(Debug, Clone, PartialEq)]
pub struct KeyMetadata {
    version: i32,
    create_time: Option<SecondsSinceEpoch>,
    hd_keypath: Option<String>,
    seed_fp: Option<Blob32>,
}

impl KeyMetadata {
    pub fn new(
        version: i32,
        create_time: Option<SecondsSinceEpoch>,
        hd_keypath: Option<String>,
        seed_fp: Option<Blob32>
    ) -> Self {
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

    pub fn create_time(&self) -> Option<&SecondsSinceEpoch> {
        self.create_time.as_ref()
    }

    pub fn hd_keypath(&self) -> Option<&str> {
        self.hd_keypath.as_deref()
    }

    pub fn seed_fp(&self) -> Option<&Blob32> {
        self.seed_fp.as_ref()
    }
}

impl Parseable for KeyMetadata {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let version = i32::parse(parser)?;
        let create_time = SecondsSinceEpoch::parse(parser)?;
        // 0 means unknown (per `walletdb.h`)
        let create_time = if create_time.as_u64() == 0 { None } else { Some(create_time) };
        let hd_keypath: Option<String>;
        let seed_fp: Option<Blob32>;
        if version >= VERSION_WITH_HDDATA {
            hd_keypath = Some(String::parse(parser)?);
            seed_fp = Some(Blob::parse(parser)?);
        } else {
            hd_keypath = None;
            seed_fp = None;
        }
        Ok(Self::new(version, create_time, hd_keypath, seed_fp))
    }
}
