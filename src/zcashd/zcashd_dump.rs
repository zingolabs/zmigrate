use std::collections::HashMap;

use anyhow::{ bail, Result, Context };

use crate::{ BDBDump, Data, Parser };

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DBKey {
    keyname: String,
    data: Data,
}

impl std::fmt::Display for DBKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.data.is_empty() {
            write!(f, "{}", self.keyname)
        } else {
            write!(f, "{}-{}", self.keyname, hex::encode(&self.data))
        }
    }
}

impl DBKey {
    pub fn new(keyname: impl Into<String>, data: impl AsRef<Data>) -> Self {
        Self { keyname: keyname.into(), data: data.as_ref().clone() }
    }

    pub fn parse(key_data: &Data) -> Result<Self> {
        let mut parser = Parser::new(&key_data);
        let keyname = parser.parse_utf8().context("Failed to parse keyname")?;
        let data = parser.rest();
        Ok(Self { keyname, data })
    }

    pub fn keyname(&self) -> &str {
        &self.keyname
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DBValue(Data);

impl DBValue {
    pub fn new(data: Data) -> Self {
        Self(data)
    }

    pub fn as_data(&self) -> &Data {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl AsRef<Data> for DBValue {
    fn as_ref(&self) -> &Data {
        &self.0
    }
}

impl AsRef<[u8]> for DBValue {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub struct ZcashdDump {
    records: HashMap<DBKey, DBValue>,
    records_by_keyname: HashMap<String, HashMap<DBKey, DBValue>>,
}

impl ZcashdDump {
    pub fn from_bdb_dump(berkeley_dump: &BDBDump) -> Result<Self> {
        let mut records: HashMap<DBKey, DBValue> = HashMap::new();
        let mut records_by_keyname: HashMap<String, HashMap<DBKey, DBValue>> = HashMap::new();

        for (key_data, value_data) in berkeley_dump.data_records() {
            let key = DBKey::parse(key_data)?;
            let value = DBValue::new(value_data.clone());
            records.insert(key.clone(), value.clone());

            let keyname = key.keyname().to_string();
            let keyname_records = records_by_keyname.entry(keyname).or_default();
            keyname_records.insert(key, value);
        }

        Ok(ZcashdDump {
            records,
            records_by_keyname,
        })
    }

    pub fn records(&self) -> &HashMap<DBKey, DBValue> {
        &self.records
    }

    pub fn value_for_key(&self, key: &DBKey) -> Result<&DBValue> {
        match self.records.get(key) {
            Some(value) => Ok(value),
            None => bail!("No record found for key: {}", key),
        }
    }

    pub fn value_for_keyname(&self, keyname: &str) -> Result<&DBValue> {
        let key = DBKey::new(keyname.to_string(), Data::new());
        self.value_for_key(&key)
            .context(format!("No record found for keyname: {}", keyname))
    }

    pub fn records_by_keyname(&self) -> &HashMap<String, HashMap<DBKey, DBValue>> {
        &self.records_by_keyname
    }

    pub fn records_for_keyname(&self, keyname: &str) -> Result<&HashMap<DBKey, DBValue>> {
        match self.records_by_keyname.get(keyname) {
            Some(records) => Ok(records),
            None => bail!("No records found for keyname: {}", keyname),
        }
    }

    pub fn record_for_keyname(&self, keyname: &str) -> Result<(DBKey, DBValue)> {
        let records = self.records_for_keyname(keyname)?;
        if records.len() != 1 {
            bail!("Expected exactly one record for keyname: {}", keyname);
        }
        match records.iter().next() {
            Some((key, value)) => Ok((key.clone(), value.clone())),
            None => bail!("No record found for keyname: {}", keyname),
        }
    }

    fn sorted_key_names(&self) -> Vec<String> {
        let mut keynames: Vec<String> = self.records_by_keyname.keys().cloned().collect();
        keynames.sort();
        keynames
    }

    pub fn print_keyname_summary(&self) {
        for keyname in self.sorted_key_names() {
            let keys = self.records_by_keyname.get(&keyname).unwrap().keys();
            let mut min_value_size: usize = usize::MAX;
            let mut max_value_size: usize = 0;
            for key in keys.clone() {
                let value = self.records_by_keyname.get(&keyname).unwrap().get(key).unwrap();
                min_value_size = min_value_size.min(value.len());
                max_value_size = max_value_size.max(value.len());
            }

            let s = if min_value_size == max_value_size {
                format!("{}", min_value_size)
            } else {
                format!("{}...{}", min_value_size, max_value_size)
            };
            println!("{}: {} ({})", keyname, keys.len(), s);
        }
    }

    pub fn print_keys(&self) {
        for keyname in self.sorted_key_names() {
            println!("{}", keyname);
            let mut keys: Vec<DBKey> = self.records_by_keyname
                .get(&keyname)
                .unwrap()
                .keys()
                .cloned()
                .collect();
            keys.sort();
            for key in keys {
                println!("    {}", key);
                let value = self.records_by_keyname.get(&keyname).unwrap().get(&key).unwrap();
                println!("        {}: {}", value.len(), hex::encode(value));
                println!();
            }
        }
    }
}
