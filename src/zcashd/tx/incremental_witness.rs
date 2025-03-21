use anyhow::Result;

use crate::{IncrementalWitness, Parse, Parser, parse};

impl<const DEPTH: usize, Hash: Parse> Parse for IncrementalWitness<DEPTH, Hash> {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            tree: parse!(p, "tree")?,
            filled: parse!(p, "filled")?,
            cursor: parse!(p, "cursor")?,
        })
    }
}
