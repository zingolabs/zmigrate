use crate::{Blob32, SecondsSinceEpoch};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMetadata {
    version: i32,
    create_time: SecondsSinceEpoch,
    hd_keypath: String,
    seed_fp: Blob32,
}
