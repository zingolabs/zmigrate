use anyhow::{ Result, Context };
use crate::{ Blob32, Parseable, SecondsSinceEpoch };

#[derive(Debug, Clone, PartialEq)]
pub struct MnemonicHDChain {
    version: i32,
    seed_fp: Blob32,
    create_time: SecondsSinceEpoch,
    account_counter: u32,
    legacy_tkey_external_counter: u32,
    legacy_tkey_internal_counter: u32,
    legacy_sapling_key_counter: u32,
    mnemonic_seed_backup_confirmed: bool,
}

impl MnemonicHDChain {
    pub fn version(&self) -> &i32 {
        &self.version
    }

    pub fn seed_fp(&self) -> &Blob32 {
        &self.seed_fp
    }

    pub fn create_time(&self) -> &SecondsSinceEpoch {
        &self.create_time
    }

    pub fn account_counter(&self) -> u32 {
        self.account_counter
    }

    pub fn legacy_tkey_external_counter(&self) -> u32 {
        self.legacy_tkey_external_counter
    }

    pub fn legacy_tkey_internal_counter(&self) -> u32 {
        self.legacy_tkey_internal_counter
    }

    pub fn legacy_sapling_key_counter(&self) -> u32 {
        self.legacy_sapling_key_counter
    }

    pub fn mnemonic_seed_backup_confirmed(&self) -> bool {
        self.mnemonic_seed_backup_confirmed
    }
}

impl Parseable for MnemonicHDChain {
    fn parse_type() -> &'static str {
        "MnemonicHDChain"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let version = parser.parse_i32()
            .context("Parsing MnemonicHDChain version")?;
        let seed_fp = parser.parse_blob()
            .context("Parsing MnemonicHDChain seed_fp")?;
        let create_time = SecondsSinceEpoch::parse(parser)
            .context("Parsing MnemonicHDChain create_time")?;
        let account_counter = u32::parse(parser)
            .context("Parsing MnemonicHDChain account_counter")?;
        let legacy_tkey_external_counter = u32::parse(parser)
            .context("Parsing MnemonicHDChain legacy_tkey_external_counter")?;
        let legacy_tkey_internal_counter = u32::parse(parser)
            .context("Parsing MnemonicHDChain legacy_tkey_internal_counter")?;
        let legacy_sapling_key_counter = u32::parse(parser)
            .context("Parsing MnemonicHDChain legacy_sapling_key_counter")?;
        let mnemonic_seed_backup_confirmed = parser.parse_bool()
            .context("Parsing MnemonicHDChain mnemonic_seed_backup_confirmed")?;

        Ok(Self {
            version,
            seed_fp,
            create_time,
            account_counter,
            legacy_tkey_external_counter,
            legacy_tkey_internal_counter,
            legacy_sapling_key_counter,
            mnemonic_seed_backup_confirmed,
        })
    }
}
