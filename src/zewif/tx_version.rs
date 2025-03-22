use anyhow::{Result, bail};

use crate::{IntID, Parse, Parser, parse};

/// Constants for standard Zcash transaction version group IDs
pub const OVERWINTER_VERSION_GROUP_ID: IntID = IntID(0x03c48270);
pub const SAPLING_VERSION_GROUP_ID: IntID = IntID(0x892f2085);
pub const ZIP225_VERSION_GROUP_ID: IntID = IntID(0x26a7270a);
pub const ZFUTURE_VERSION_GROUP_ID: IntID = IntID(0xffffffff);

/// Constants for standard Zcash transaction versions
pub const OVERWINTER_TX_VERSION: u32 = 3;
pub const SAPLING_TX_VERSION: u32 = 4;
pub const ZIP225_TX_VERSION: u32 = 5;
pub const ZFUTURE_TX_VERSION: u32 = 0x0000ffff;

/// Represents the version group of a Zcash transaction.
/// Each version group corresponds to a specific network upgrade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TxVersionGroup {
    /// Pre-Overwinter transactions (Sprout era)
    PreOverwinter,
    /// Overwinter network upgrade (version 3)
    OverwinterV3,
    /// Sapling network upgrade (version 4)
    SaplingV4,
    /// ZIP-225 network upgrade (version 5, support for Orchard)
    Zip225V5,
    /// Future network upgrades
    Future,
}

/// Represents the version of a Zcash transaction.
/// Combines both the numeric version and the version group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TxVersion {
    group: TxVersionGroup,
    number: u32,
}

impl TxVersion {
    /// Get the version group of this transaction
    pub fn group(&self) -> TxVersionGroup {
        self.group
    }

    /// Get the numeric version of this transaction
    pub fn number(&self) -> u32 {
        self.number
    }

    /// Returns true if this transaction is from the Overwinter era or later
    pub fn is_overwinter(&self) -> bool {
        self.group != TxVersionGroup::PreOverwinter
    }

    /// Returns true if this transaction is from the Sapling era
    pub fn is_sapling(&self) -> bool {
        self.group == TxVersionGroup::SaplingV4
    }

    /// Returns true if this transaction is from the ZIP-225 era (with Orchard support)
    pub fn is_zip225(&self) -> bool {
        self.group == TxVersionGroup::Zip225V5
    }

    /// Returns true if this transaction is from a future network upgrade
    pub fn is_future(&self) -> bool {
        self.group == TxVersionGroup::Future
    }
}

impl Parse for TxVersion {
    fn parse(p: &mut Parser) -> Result<Self> {
        let header: u32 = parse!(p, "Transaction header")?;
        let overwintered = (header >> 31) == 1;
        let number = header & 0x7fffffff;

        let version_group_id = overwintered
            .then(|| parse!(p, "Transaction version group ID"))
            .transpose()?
            .unwrap_or_default();

        let group = match (overwintered, version_group_id, number) {
            (false, _, _) => TxVersionGroup::PreOverwinter,
            (true, OVERWINTER_VERSION_GROUP_ID, OVERWINTER_TX_VERSION) => {
                TxVersionGroup::OverwinterV3
            }
            (true, SAPLING_VERSION_GROUP_ID, SAPLING_TX_VERSION) => TxVersionGroup::SaplingV4,
            (true, ZIP225_VERSION_GROUP_ID, ZIP225_TX_VERSION) => TxVersionGroup::Zip225V5,
            (true, ZFUTURE_VERSION_GROUP_ID, ZFUTURE_TX_VERSION) => TxVersionGroup::Future,
            _ => bail!(
                "Unsupported transaction format: overwintered={}, version={}, version_group_id={}",
                overwintered,
                number,
                version_group_id
            ),
        };

        Ok(Self { group, number })
    }
}
