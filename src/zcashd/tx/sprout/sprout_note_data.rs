use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

use super::{SproutPaymentAddress, SproutWitness};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SproutNoteData {
    pub address: SproutPaymentAddress,
    pub nullifer: Option<u256>,
    pub witnesses: Vec<SproutWitness>,
    pub witness_height: i32,
}

impl Parse for SproutNoteData {
    fn parse(p: &mut Parser) -> Result<Self> {
        let address = parse!(p, "sprout note data address")?;
        let nullifer = parse!(p, "sprout note data nullifier")?;
        let witnesses = parse!(p, "sprout note data witnesses")?;
        let witness_height = parse!(p, "sprout note data witness height")?;
        Ok(Self {
            address,
            nullifer,
            witnesses,
            witness_height,
        })
    }
}
