use anyhow::{Result, Context};
use zcash_client_backend::proto::compact_formats::CompactBlock;
use zcash_primitives::merkle_tree::read_commitment_tree;
use prost::Message;

use crate::{parse, Blob32, Data, Parse, Parser};

#[derive(Debug, Clone)]
pub struct BlockData {
    pub ecb: Data,
    pub height: u64,
}

impl BlockData {
    pub(crate) fn new_with(height: u64, hash: impl AsRef<[u8]>) -> Self {
        let hash = hash.as_ref().iter().copied().rev().collect::<Vec<_>>();

        let cb = CompactBlock {
            hash,
            ..Default::default()
        };

        let mut ecb = vec![];
        cb.encode(&mut ecb).unwrap();

        Self { ecb: Data(ecb), height }
    }
}

impl Parse for BlockData {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        let height = parse!(p, i32, "height")? as u64;

        let mut hash_bytes = parse!(p, Blob32, "hash")?;
        hash_bytes.reverse();

        let tree: sapling_crypto::CommitmentTree = read_commitment_tree(&mut *p).with_context(|| "CommitmentTree")?;
        let _tree = if tree.size() == 0 { None } else { Some(tree) };

        let version = parse!(p, u64, "version")?;

        let ecb = if version <= 11 {
            vec![]
        } else {
            parse!(p, Vec<u8>, "ecb")?
        };

        if ecb.is_empty() {
            Ok(BlockData::new_with(height, &hash_bytes))
        } else {
            Ok(BlockData { ecb: Data(ecb), height })
        }
    }
}
