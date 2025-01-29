use std::collections::HashMap;

use anyhow::{ Context, Result };

use crate::{ Parseable, Parser };

use super::{ KeyPair, Keys, PrivKey, PubKey, ZcashdDump, ZcashdWallet };

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
        let version = self.parse_version()?;
        let keys = self.parse_keys()?;
        Ok(ZcashdWallet::new(version, keys))
    }

    fn parse_version(&self) -> Result<i32> {
        let mut parser = Parser::new(self.dump.value_for_keyname("version")?);
        parser.parse_i32().context("Failed to parse version")
    }

    fn parse_keys(&self) -> Result<Keys> {
        let records_for_keyname = self.dump
            .records_for_keyname("key")
            .context("Failed to get 'key' records")?;
        let mut keys_map = HashMap::new();
        for (key, value) in records_for_keyname {
            let pubkey = PubKey::parse_binary(&key.data()).context("Failed to parse pubkey")?;
            let privkey = PrivKey::parse_binary(&value.as_data()).context(
                "Failed to parse privkey"
            )?;
            let keypair = KeyPair::new(pubkey.clone(), privkey.clone())
                .context("Failed to create keypair")?;
            keys_map.insert(pubkey, keypair);
        }
        Ok(Keys::new(keys_map))
    }
}
