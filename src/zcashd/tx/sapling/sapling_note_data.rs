use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

use crate::{SaplingIncomingViewingKey, SaplingWitness};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingNoteData {
    pub version: i32,
    pub ivk: SaplingIncomingViewingKey,
    pub nullifer: Option<u256>,
    pub witnesses: Vec<SaplingWitness>,
    pub witness_height: i32,
}

impl Parse for SaplingNoteData {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version = parse!(p, "sapling note data version")?;
        let ivk = parse!(p, "sapling note data ivk")?;
        let nullifer = parse!(p, "sapling note data nullifier")?;
        let witnesses = parse!(p, "sapling note data witnesses")?;
        let witness_height = parse!(p, "sapling note data witness height")?;
        Ok(Self {
            version,
            ivk,
            nullifer,
            witnesses,
            witness_height,
        })
    }
}
