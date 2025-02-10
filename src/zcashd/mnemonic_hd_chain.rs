use anyhow::Result;
use crate::{ parse, Blob32, Parse, Parser, SecondsSinceEpoch };

#[derive(Debug, Clone, PartialEq)]
pub struct MnemonicHDChain {
    pub version: i32,
    pub seed_fp: Blob32,
    pub create_time: SecondsSinceEpoch,
    pub account_counter: u32,
    pub legacy_tkey_external_counter: u32,
    pub legacy_tkey_internal_counter: u32,
    pub legacy_sapling_key_counter: u32,
    pub mnemonic_seed_backup_confirmed: bool,
}

impl Parse for MnemonicHDChain {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version = parse!(p, "version")?;
        let seed_fp = parse!(p, "seed_fp")?;
        let create_time = parse!(p, "create_time")?;
        let account_counter = parse!(p, "account_counter")?;
        let legacy_tkey_external_counter = parse!(p, "legacy_tkey_external_counter")?;
        let legacy_tkey_internal_counter = parse!(p, "legacy_tkey_internal_counter")?;
        let legacy_sapling_key_counter = parse!(p, "legacy_sapling_key_counter")?;
        let mnemonic_seed_backup_confirmed = parse!(p, "mnemonic_seed_backup_confirmed")?;

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
