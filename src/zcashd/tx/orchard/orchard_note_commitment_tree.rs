use crate::{Data, Parser};
use anyhow::Result;
use crate::Parse;

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardNoteCommitmentTree {
    pub rest: Data
}

impl Parse for OrchardNoteCommitmentTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            rest: p.rest()
        })
    }
}
