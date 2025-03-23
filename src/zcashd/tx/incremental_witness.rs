use anyhow::Result;

use crate::{IncrementalWitness, Parse, Parser, parse};

impl<const DEPTH: usize, Hash: Parse> Parse for IncrementalWitness<DEPTH, Hash> {
    fn parse(p: &mut Parser) -> Result<Self> {
        let tree = parse!(p, "tree")?;
        let filled = parse!(p, "filled")?;
        let cursor = parse!(p, "cursor")?;
        Ok(Self::with_fields(tree, filled, cursor))
    }
}
