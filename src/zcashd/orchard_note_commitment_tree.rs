use crate::Data;
use anyhow::Result;
use crate::Parseable;

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardNoteCommitmentTree(Data);

impl OrchardNoteCommitmentTree {
    pub fn data(&self) -> &Data {
        &self.0
    }
}

impl Parseable for OrchardNoteCommitmentTree {
    fn parse_type() -> &'static str {
        "OrchardNoteCommitmentTree"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let data = parser.rest();
        Ok(Self(data))
    }
}
