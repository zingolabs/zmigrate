use crate::{Data, Parser};
use anyhow::Result;
use crate::Parse;

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardNoteCommitmentTree(Data);

impl OrchardNoteCommitmentTree {
    pub fn data(&self) -> &Data {
        &self.0
    }
}

impl Parse for OrchardNoteCommitmentTree {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        let data = p.rest();
        Ok(Self(data))
    }
}
