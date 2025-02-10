use std::collections::HashMap;

use anyhow::{ Context, Result, bail };

use crate::{ parse, u256 };

use super::{
    zcashd_dump::DBKey, Address, BlockLocator, ClientVersion, Key, KeyMetadata, KeyPoolEntry, Keys, MnemonicHDChain, MnemonicSeed, NetworkInfo, OrchardNoteCommitmentTree, PrivKey, PubKey, WalletTx, ZcashdDump, ZcashdWallet
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
        let address_names = self.parse_address_names()?;

        // **orderposnext**
        let orderposnext = self.parse_opt_i64("orderposnext")?;

        // pool
        let key_pool = self.parse_key_pool()?;

        // purpose
        let address_purposes = self.parse_address_purposes()?;

        // sapzaddr

        // sapextfvk

        // sapzkey

        // tx
        let transactions = self.parse_transactions()?;

        // **version**
        let client_version = self.parse_client_version("version")?;

        // vkey

        // watchs

        // **witnesscachesize**
        let witnesscachesize = self.parse_i64("witnesscachesize")?;

        // wkey

        // zkey

        // zkeymeta

        //
        // Since version 5
        //

        // **networkinfo**
        let network_info = self.parse_network_info()?;

        // **orchard_note_commitment_tree**
        let orchard_note_commitment_tree = self.parse_orchard_note_commitment_tree()?;

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

        Ok(
            ZcashdWallet::new(
                bestblock_nomerkle,
                bestblock,
                client_version,
                default_key,
                keys,
                min_version,
                mnemonic_hd_chain,
                mnemonic_phrase,
                address_names,
                address_purposes,
                network_info,
                orchard_note_commitment_tree,
                orderposnext,
                witnesscachesize,
                key_pool,
                transactions,
            )
        )
    }

    fn parse_i64(&self, keyname: &str) -> Result<i64> {
        let value = self.dump.value_for_keyname(keyname)?;
        parse!(buf value, i64, format!("i64 for keyname: {}", keyname))
    }

    fn parse_opt_i64(&self, keyname: &str) -> Result<Option<i64>> {
        if self.dump.has_value_for_keyname(keyname) {
            self.parse_i64(keyname).map(Some)
        } else {
            Ok(None)
        }
    }

    fn parse_client_version(&self, keyname: &str) -> Result<ClientVersion> {
        let value = self.dump.value_for_keyname(keyname)?;
        parse!(buf value, ClientVersion, format!("client version for keyname: {}", keyname))
    }

    fn parse_block_locator(&self, keyname: &str) -> Result<BlockLocator> {
        let value = self.dump.value_for_keyname(keyname)?;
        parse!(buf value, BlockLocator, format!("block locator for keyname: {}", keyname))
    }

    fn parse_keys(&self) -> Result<Keys> {
        let key_records = self.dump.records_for_keyname("key").context("Getting 'key' records")?;
        let keymeta_records = self.dump
            .records_for_keyname("keymeta")
            .context("Getting 'keymeta' records")?;
        if key_records.len() != keymeta_records.len() {
            bail!("Mismatched key and keymeta records");
        }
        let mut keys_map = HashMap::new();
        for (key, value) in key_records {
            let pubkey = parse!(buf &key.data, PubKey, "pubkey")?;
            let privkey = parse!(buf value.as_data(), PrivKey, "privkey")?;
            let metakey = DBKey::new("keymeta", &key.data);
            let metadata_binary = self.dump.value_for_key(&metakey).context("Getting metadata")?;
            let metadata = parse!(buf metadata_binary, KeyMetadata, "metadata")?;
            let keypair = Key::new(pubkey.clone(), privkey.clone(), metadata).context(
                "Creating keypair"
            )?;
            keys_map.insert(pubkey, keypair);
        }
        Ok(Keys::new(keys_map))
    }

    fn parse_default_key(&self) -> Result<PubKey> {
        let value = self.dump.value_for_keyname("defaultkey")?;
        parse!(buf value, PubKey, "defaultkey")
    }

    fn parse_mnemonic_hd_chain(&self) -> Result<MnemonicHDChain> {
        let value = self.dump.value_for_keyname("mnemonichdchain")?;
        parse!(buf value, MnemonicHDChain, "mnemonichdchain")
    }

    fn parse_mnemonic_phrase(&self) -> Result<MnemonicSeed> {
        let (key, value) = self.dump
            .record_for_keyname("mnemonicphrase")
            .context("Getting 'mnemonicphrase' record")?;
        let fingerprint = parse!(buf &key.data, u256, "seed fingerprint")?;
        let seed = parse!(buf &value, MnemonicSeed, "mnemonic phrase")?
            .set_fingerprint(fingerprint);
        Ok(seed)
    }

    fn parse_address_names(&self) -> Result<HashMap<Address, String>> {
        let records = self.dump.records_for_keyname("name").context("Getting 'name' records")?;
        let mut address_names = HashMap::new();
        for (key, value) in records {
            let address = parse!(buf &key.data, Address, "address")?;
            let name = parse!(buf value.as_data(), String, "name")?;
            if address_names.contains_key(&address) {
                bail!("Duplicate address found: {}", address);
            }
            address_names.insert(address, name);
        }
        Ok(address_names)
    }

    fn parse_address_purposes(&self) -> Result<HashMap<Address, String>> {
        let records = self.dump.records_for_keyname("purpose").context("Getting 'purpose' records")?;
        let mut address_purposes = HashMap::new();
        for (key, value) in records {
            let address = parse!(buf &key.data, Address, "address")?;
            let purpose = parse!(buf value.as_data(), String, "purpose")?;
            if address_purposes.contains_key(&address) {
                bail!("Duplicate address found: {}", address);
            }
            address_purposes.insert(address, purpose);
        }
        Ok(address_purposes)
    }

    fn parse_network_info(&self) -> Result<NetworkInfo> {
        let (_, value) = self.dump
            .record_for_keyname("networkinfo")
            .context("Getting 'networkinfo' record")?;
        let network_info = parse!(buf value.as_data(), NetworkInfo, "network info")?;
        Ok(network_info)
    }

    fn parse_orchard_note_commitment_tree(&self) -> Result<OrchardNoteCommitmentTree> {
        let (_, value) = self.dump
            .record_for_keyname("orchard_note_commitment_tree")
            .context("Getting 'orchard_note_commitment_tree' record")?;
        let orchard_note_commitment_tree = parse!(buf value.as_data(), OrchardNoteCommitmentTree, "orchard note commitment tree")?;
        Ok(orchard_note_commitment_tree)
    }

    fn parse_key_pool(&self) -> Result<HashMap<i64, KeyPoolEntry>> {
        let records = self.dump.records_for_keyname("pool").context("Getting 'pool' records")?;
        let mut key_pool = HashMap::new();
        for (key, value) in records {
            let index = parse!(buf &key.data, i64, "key pool index")?;
            let entry = parse!(buf value.as_data(), KeyPoolEntry, "key pool entry")?;
            key_pool.insert(index, entry);
        }
        Ok(key_pool)
    }

    fn parse_transactions(&self) -> Result<HashMap<u256, WalletTx>> {
        let mut transactions = HashMap::new();
        // Some wallet files don't have any transactions
        if self.dump.has_records_for_keyname("tx") {
            let records = self.dump.records_for_keyname("tx").context("Getting 'tx' records")?;
            for (key, value) in records {
                let txid = parse!(buf &key.data, u256, "transaction ID")?;
                let transaction = parse!(buf value.as_data(), WalletTx, "transaction")?;
                if transactions.contains_key(&txid) {
                    bail!("Duplicate transaction found: {:?}", txid);
                }
                transactions.insert(txid, transaction);
            }
        }
        Ok(transactions)
    }
}
