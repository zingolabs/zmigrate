use anyhow::{bail, Result, Context};

use crate::{Parse, Parser};

/// A fixed-size byte array.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Blob<const N: usize>([u8; N]);

impl<const N: usize> Blob<N> {
    pub fn new(data: [u8; N]) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_slice(data: &[u8]) -> Result<Self> {
        if data.len() != N {
            bail!("Invalid data length: expected {}, got {}", N, data.len());
        }
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(data);
        Ok(Self::new(bytes))
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Self::from_slice(&data)
    }
}

impl<const N: usize> AsRef<[u8]> for Blob<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> std::fmt::Debug for Blob<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Blob<{}>({})", N, hex::encode(self.0))
    }
}

impl<const N: usize> From<Blob<N>> for Vec<u8> {
    fn from(blob: Blob<N>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const N: usize> From<&Blob<N>> for Vec<u8> {
    fn from(blob: &Blob<N>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const N: usize> From<Vec<u8>> for Blob<N> {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data).unwrap()
    }
}

impl<const N: usize> From<&[u8]> for Blob<N> {
    fn from(data: &[u8]) -> Self {
        Self::from_vec(data.to_vec()).unwrap()
    }
}

impl<const N: usize> Parse for Blob<N> {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let data = parser.next(N).with_context(|| format!("Parsing Blob<{}>", N))?;
        Self::from_slice(data)
    }
}

pub type Blob20 = Blob<20>;
pub type Blob32 = Blob<32>;
