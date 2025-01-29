use std::collections::HashMap;

use anyhow::{ Context, Result };

use crate::{Blob32, Parseable};

use super::{
    zcashd_dump::DBKey, BlockLocator, ClientVersion, Key, KeyMetadata, Keys, MnemonicHDChain, MnemonicSeed, PrivKey, PubKey, ZcashdDump, ZcashdWallet
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
        //
        // Since version 3
        //

        // ~~acc~~: Removed in 4.5.0
        // ~~acentry~~: Removed in 4.5.0

        // **bestblock**: Empty in 6.0.0
        let bestblock = self.parse_block_locator("bestblock")?;

        // ~~**chdseed**~~: Removed in 5.0.0

        // ckey

        // csapzkey

        // cscript

        // czkey

        // **defaultkey**
        let default_key = self.parse_default_key()?;

        // destdata

        // **hdchain**

        // ~~hdseed~~: Removed in 5.0.0

        // key
        // keymeta
        let keys = self.parse_keys()?;

        // **minversion**
        let min_version = self.parse_client_version("minversion")?;

        // **mkey**

        // name

        // **orderposnext**

        // pool

        // purpose

        // sapzaddr

        // sapextfvk

        // sapzkey

        // tx

        // **version**
        let client_version = self.parse_client_version("version")?;

        // vkey

        // watchs

        // **witnesscachesize**

        // wkey

        // zkey

        // zkeymeta

        //
        // Since version 5
        //

        // **networkinfo**

        // **orchard_note_commitment_tree**

        // unifiedaccount

        // unifiedfvk

        // unifiedaddrmeta

        // **mnemonicphrase**
        let mnemonic_phrase = self.parse_mnemonic_phrase()?;

        // **cmnemonicphrase**

        // **mnemonichdchain**
        let mnemonic_hd_chain = self.parse_mnemonic_hd_chain()?;

        // recipientmapping

        //
        // Since version 6
        //

        // **bestblock_nomerkle**
        let bestblock_nomerkle = self.parse_block_locator("bestblock_nomerkle")?;

        Ok(ZcashdWallet::new(
            bestblock_nomerkle,
            bestblock,
            client_version,
            default_key,
            keys,
            min_version,
            mnemonic_hd_chain,
            mnemonic_phrase,
        ))
    }

    fn parse_client_version(&self, keyname: &str) -> Result<ClientVersion> {
        let value = self.dump.value_for_keyname(keyname)?;
        ClientVersion::parse_binary(value)
            .context(format!("Failed to parse client version for keyname: {}", keyname))
    }

    fn parse_block_locator(&self, keyname: &str) -> Result<BlockLocator> {
        let value = self.dump.value_for_keyname(keyname)?;
        BlockLocator::parse_binary(value)
            .context(format!("Failed to parse block locator for keyname: {}", keyname))
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

    fn parse_mnemonic_hd_chain(&self) -> Result<MnemonicHDChain> {
        let value = self.dump.value_for_keyname("mnemonichdchain")?;
        MnemonicHDChain::parse_binary(value)
    }

    fn parse_mnemonic_phrase(&self) -> Result<MnemonicSeed> {
        let (key, value) = self.dump.record_for_keyname("mnemonicphrase")
            .context("Failed to get 'mnemonicphrase' record")?;
        let fingerprint = Blob32::parse_binary(key.data())
            .context("Failed to parse seed fingerprint")?;
        let seed = MnemonicSeed::parse_binary(&value)
            .context("Failed to parse mnemonic phrase")?
            .set_fingerprint(fingerprint);
        Ok(seed)
    }
}
