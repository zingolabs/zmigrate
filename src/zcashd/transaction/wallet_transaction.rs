use anyhow::{ Result, Context, bail };
use crate::{ Data, Parseable };

use super::IntID;

const OVERWINTER_VERSION_GROUP_ID: IntID = IntID::new(0x03c48270);
const OVERWINTER_TX_VERSION: u32 = 3;
const SAPLING_VERSION_GROUP_ID: IntID = IntID::new(0x892f2085);
const SAPLING_TX_VERSION: u32 = 4;
const ZIP225_VERSION_GROUP_ID: IntID = IntID::new(0x26a7270a);
const ZIP225_TX_VERSION: u32 = 5;
const ZFUTURE_VERSION_GROUP_ID: IntID = IntID::new(0xffffffff);
const ZFUTURE_TX_VERSION: u32 = 0x0000ffff;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WalletTransaction {
    pub overwintered: bool,
    pub version_group_id: IntID,
    pub version: u32,
    pub rest: Data,
}

impl WalletTransaction {}

impl Parseable for WalletTransaction {
    fn parse_type() -> &'static str {
        "Transaction"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let header = parser.parse_u32().context("Parsing Transaction header")?;
        let overwintered = (header >> 31) == 1;
        let version = header & 0x7fffffff;

        let version_group_id = if overwintered {
            IntID::parse(parser)
                .context("Parsing Transaction version group ID")?
        } else {
            IntID::default()
        };

        let is_overwinter_v3 =
            overwintered &&
            version_group_id == OVERWINTER_VERSION_GROUP_ID &&
            version == OVERWINTER_TX_VERSION;

        let is_sapling_v4 =
            overwintered &&
            version_group_id == SAPLING_VERSION_GROUP_ID &&
            version == SAPLING_TX_VERSION;

        let is_zip225_v5 =
            overwintered &&
            version_group_id == ZIP225_VERSION_GROUP_ID &&
            version == ZIP225_TX_VERSION;

        let is_future =
            overwintered &&
            version_group_id == ZFUTURE_VERSION_GROUP_ID &&
            version == ZFUTURE_TX_VERSION;

        if !is_overwinter_v3 && !is_sapling_v4 && !is_zip225_v5 && !is_future {
            bail!("Unsupported transaction format: overwintered={}, version={}, version_group_id={}", overwintered, version, version_group_id);
        }

        let rest = parser.rest();
        Ok(Self {
            overwintered,
            version_group_id,
            version,
            rest,
        })
    }
}
