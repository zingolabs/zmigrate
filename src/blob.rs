use anyhow::{bail, Result};

/// Represents a fixed-size byte array.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Blob<const T: usize>([u8; T]);

impl<const T: usize> Blob<T> {
    pub fn new(data: [u8; T]) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        T
    }

    pub fn is_empty(&self) -> bool {
        T == 0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_slice(data: &[u8]) -> Result<Self> {
        if data.len() != T {
            bail!("Invalid data length: expected {}, got {}", T, data.len());
        }
        let mut bytes = [0u8; T];
        bytes.copy_from_slice(data);
        Ok(Self::new(bytes))
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Self::from_slice(&data)
    }
}

impl<const T: usize> AsRef<[u8]> for Blob<T> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const T: usize> std::fmt::Debug for Blob<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Blob<{}>({})", T, hex::encode(self.0))
    }
}

impl<const T: usize> From<Blob<T>> for Vec<u8> {
    fn from(blob: Blob<T>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const T: usize> From<&Blob<T>> for Vec<u8> {
    fn from(blob: &Blob<T>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const T: usize> From<Vec<u8>> for Blob<T> {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data).unwrap()
    }
}

impl<const T: usize> From<&[u8]> for Blob<T> {
    fn from(data: &[u8]) -> Self {
        Self::from_vec(data.to_vec()).unwrap()
    }
}

pub type Blob20 = Blob<20>;
pub type Blob32 = Blob<32>;
