use sha2::{Digest, Sha256};

use crate::Blob32;

/// SHA256 hash.
pub fn sha256(data: impl AsRef<[u8]>) -> Blob32 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    Blob32::from_slice(&result).unwrap()
}

/// Bitcoin double SHA256 hash.
pub fn hash256(data: impl AsRef<[u8]>) -> Blob32 {
    sha256(sha256(data))
}
