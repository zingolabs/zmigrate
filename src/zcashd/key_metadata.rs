use anyhow::Result;

use crate::{parse, Blob32, Parse, Parser, SecondsSinceEpoch};

const VERSION_WITH_HDDATA: i32 = 10;
#[derive(Debug, Clone, PartialEq)]
pub struct KeyMetadata {
    pub version: i32,
    pub create_time: Option<SecondsSinceEpoch>,
    pub hd_keypath: Option<String>,
    pub seed_fp: Option<Blob32>,
}

impl Parse for KeyMetadata {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version = parse!(p, "version")?;
        let create_time: SecondsSinceEpoch = parse!(p, "create_time")?;
        // 0 means unknown (per `walletdb.h`)
        let create_time = if create_time.is_zero() {
            None
        } else {
            Some(create_time)
        };
        let hd_keypath: Option<String>;
        let seed_fp: Option<Blob32>;
        if version >= VERSION_WITH_HDDATA {
            hd_keypath = Some(parse!(p, "hd_keypath")?);
            seed_fp = Some(parse!(p, "seed_fp")?);
        } else {
            hd_keypath = None;
            seed_fp = None;
        }
        Ok(Self { version, create_time, hd_keypath, seed_fp })
    }
}
