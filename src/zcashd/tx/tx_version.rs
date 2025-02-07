use anyhow::{Result, Context, bail};

use crate::Parseable;

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
pub enum TxVersionGroup {
    PreOverwinter,
    OverwinterV3,
    SaplingV4,
    Zip225V5,
    Future,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TxVersion {
    pub group: TxVersionGroup,
    pub version: u32,
}

impl Parseable for TxVersion {
    fn parse_type() -> &'static str {
        "TxVersion"
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

        let group = match (overwintered, version_group_id, version) {
            (false, _, _) => TxVersionGroup::PreOverwinter,
            (true, OVERWINTER_VERSION_GROUP_ID, OVERWINTER_TX_VERSION) => TxVersionGroup::OverwinterV3,
            (true, SAPLING_VERSION_GROUP_ID, SAPLING_TX_VERSION) => TxVersionGroup::SaplingV4,
            (true, ZIP225_VERSION_GROUP_ID, ZIP225_TX_VERSION) => TxVersionGroup::Zip225V5,
            (true, ZFUTURE_VERSION_GROUP_ID, ZFUTURE_TX_VERSION) => TxVersionGroup::Future,
            _ => bail!("Unsupported transaction format: overwintered={}, version={}, version_group_id={}", overwintered, version, version_group_id),
        };

        Ok(Self {
            group,
            version,
        })
    }
}
