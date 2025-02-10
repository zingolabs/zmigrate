use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

use crate::{SaplingIncomingViewingKey, SaplingWitness};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingNoteData {
    pub version: i32,
    pub incoming_viewing_key: SaplingIncomingViewingKey,
    pub nullifer: Option<u256>,
    pub witnesses: Vec<SaplingWitness>,
    pub witness_height: i32,
}

impl Parse for SaplingNoteData {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            version: parse!(p, "version")?,
            incoming_viewing_key: parse!(p, "incoming_viewing_key")?,
            nullifer: parse!(p, "nullifer")?,
            witnesses: parse!(p, "witnesses")?,
            witness_height: parse!(p, "witness_height")?,
        })
    }
}
