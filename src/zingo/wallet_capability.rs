#![allow(unused_variables, unused_assignments)]

use anyhow::bail;

use crate::{blob, parse, sapling::SaplingExtendedSpendingKey, zingo::{UnifiedKeystore, Version2Keystore}, Parse, Parser};

use super::{Keystore, ReceiverSelection, Versioned, Capability, LegacyExtendedPrivKey, LegacyExtendedPubKey, SaplingDiversifiableFullViewingKey, Version1Keystore};

blob!(OrchardFullViewingKey, 96);
blob!(OrchardSpendingKey, 32);

#[derive(Debug, Clone, Default)]
pub struct WalletCapability {
    pub version: u8,
    pub keystore: Keystore,
    pub receiver_selections: Vec<ReceiverSelection>,
    pub length_of_rejection_addresses: u32,
}

impl Versioned for WalletCapability {
    const VERSION: u8 = 4;
}

impl Parse for WalletCapability {
    fn parse(p: &mut Parser) -> anyhow::Result<Self> where Self: Sized {
        let version = Self::get_version(p)?;

        let legacy_key: bool;
        let length_of_rejection_addresses: u32;

        #[allow(unreachable_patterns, unused_variables, unreachable_code, clippy::let_unit_value)]
        let wc = match version {
            1 => {
                legacy_key = true;
                length_of_rejection_addresses = 0;

                let keystore = parse!(p, Version1Keystore, "Version1Keystore")?;
                Self {
                    keystore: Keystore::Version1(Box::new(keystore)),
                    ..Default::default()
                }
            }
            2 => {
                legacy_key = true;
                length_of_rejection_addresses = 0;

                let keystore = parse!(p, Version2Keystore, "Version2Keystore")?;
                Self {
                    keystore: Keystore::Version2(Box::new(keystore)),
                    ..Default::default()
                }
            }
            3 => {
                legacy_key = false;
                length_of_rejection_addresses = 0;

                let keystore = parse!(p, UnifiedKeystore, "UnifiedKeystore")?;
                Self {
                    keystore: Keystore::Unified(Box::new(keystore)),
                    ..Default::default()
                }
            }
            4 => {
                legacy_key = false;
                length_of_rejection_addresses = parse!(p, u32, "length_of_rejection_addresses")?;

                let keystore = parse!(p, UnifiedKeystore, "UnifiedKeystore")?;
                Self {
                    keystore: Keystore::Unified(Box::new(keystore)),
                    ..Default::default()
                }
            }
            _ => {
                bail!("Unknown WalletCapability version");
            }
        };

        let receiver_selections = parse!(p, "receiver_selections")?;
        Ok(Self {
            version,
            receiver_selections,
            length_of_rejection_addresses,
            ..wc
        })
    }
}
