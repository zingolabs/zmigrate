use anyhow::{ Result, Context };

use crate::Parser;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A variable-size byte array.
pub struct Data(Vec<u8>);

impl Data {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_bytes(data: impl AsRef<[u8]>) -> Self {
        Self(data.as_ref().to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_slice(data: &[u8]) -> Self {
        Self(data.to_vec())
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        Ok(Self(hex::decode(hex)?))
    }

    pub fn concat(a: &[&dyn AsRef<[u8]>]) -> Self {
        let mut bytes = Vec::new();
        for data in a {
            bytes.extend_from_slice(data.as_ref());
        }
        Self(bytes)
    }
}

impl Data {
    pub fn parse_len(parser: &mut Parser, len: usize) -> Result<Self> {
        let bytes = parser.next(len).context("Parsing Data")?;
        Ok(Self::from_slice(bytes))
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data<{}>({})", self.len(), hex::encode(self))
    }
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        self
    }
}

impl From<Data> for Vec<u8> {
    fn from(data: Data) -> Vec<u8> {
        data.to_vec()
    }
}

impl From<&Data> for Vec<u8> {
    fn from(data: &Data) -> Vec<u8> {
        data.to_vec()
    }
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data)
    }
}

impl From<&[u8]> for Data {
    fn from(data: &[u8]) -> Self {
        Self::from_slice(data)
    }
}
