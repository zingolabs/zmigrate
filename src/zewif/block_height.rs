use crate::{Parse, parse};

pub type BlockHeight = zcash_primitives::consensus::BlockHeight;

impl Parse for BlockHeight {
    fn parse(p: &mut crate::Parser) -> anyhow::Result<Self> {
        let height = parse!(p, u32, "BlockHeight")?;
        Ok(BlockHeight::from(height))
    }
}
