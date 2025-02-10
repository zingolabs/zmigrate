use crate::{Data, Parser};
use anyhow::Result;
use crate::Parse;

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardNoteCommitmentTree(pub Data);

impl Parse for OrchardNoteCommitmentTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        let data = p.rest();
        Ok(Self(data))
    }
}
