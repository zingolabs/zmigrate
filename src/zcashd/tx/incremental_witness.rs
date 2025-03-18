use anyhow::Result;

use crate::{parse, IncrementalWitness, Parse, Parser};

impl<const DEPTH: usize, Hash: Parse> Parse for IncrementalWitness<DEPTH, Hash> {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            tree: parse!(p, "tree")?,
            filled: parse!(p, "filled")?,
            cursor: parse!(p, "cursor")?,
        })
    }
}
