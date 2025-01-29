use std::collections::HashMap;

use anyhow::{ Context, Result };

use crate::Parseable;

use super::{
    zcashd_dump::DBKey, ClientVersion, Key, KeyMetadata, Keys, PrivKey, PubKey, ZcashdDump, ZcashdWallet
};

#[derive(Debug)]
pub struct ZcashdParser<'a> {
    dump: &'a ZcashdDump,
}

impl<'a> ZcashdParser<'a> {
    pub fn parse_dump(dump: &ZcashdDump) -> Result<ZcashdWallet> {
        let parser = ZcashdParser::new(dump);
        parser.parse()
    }

    fn new(dump: &'a ZcashdDump) -> Self {
        Self { dump }
    }

    fn parse(&self) -> Result<ZcashdWallet> {
        let version = self.parse_client_version("version")?;
        let min_version = self.parse_client_version("minversion")?;
        let default_key = self.parse_default_key()?;
        let keys = self.parse_keys()?;
        Ok(ZcashdWallet::new(
            version,
            min_version,
            default_key,
            keys,
        ))
    }

    fn parse_client_version(&self, keyname: &str) -> Result<ClientVersion> {
        let value = self.dump.value_for_keyname(keyname)?;
        ClientVersion::parse_binary(value)
    }

    fn parse_keys(&self) -> Result<Keys> {
        let key_records = self.dump
            .records_for_keyname("key")
            .context("Failed to get 'key' records")?;
        let keymeta_records = self.dump
            .records_for_keyname("keymeta")
            .context("Failed to get 'keymeta' records")?;
        if key_records.len() != keymeta_records.len() {
            anyhow::bail!("Mismatched key and keymeta records");
        }
        let mut keys_map = HashMap::new();
        for (key, value) in key_records {
            let pubkey = PubKey::parse_binary(&key.data()).context("Failed to parse pubkey")?;
            let privkey = PrivKey::parse_binary(&value.as_data()).context(
                "Failed to parse privkey"
            )?;
            let metakey = DBKey::new("keymeta", key.data());
            let metadata_binary = self.dump
                .value_for_key(&metakey)
                .context("Failed to get metadata")?;
            let metadata = KeyMetadata::parse_binary(&metadata_binary).context(
                "Failed to parse metadata"
            )?;
            let keypair = Key::new(pubkey.clone(), privkey.clone(), metadata).context(
                "Failed to create keypair"
            )?;
            keys_map.insert(pubkey, keypair);
        }
        Ok(Keys::new(keys_map))
    }

    fn parse_default_key(&self) -> Result<PubKey> {
        let value = self.dump.value_for_keyname("defaultkey")?;
        PubKey::parse_binary(value)
    }
}
