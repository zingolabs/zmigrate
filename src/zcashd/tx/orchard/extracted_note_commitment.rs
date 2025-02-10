use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::Fp;

#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedNoteCommitment(Fp);

impl Parse for ExtractedNoteCommitment {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self(parse!(p, "ExtractedNoteCommitment")?))
    }
}
