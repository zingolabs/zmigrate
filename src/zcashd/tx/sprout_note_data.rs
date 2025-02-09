use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

use super::{SproutPaymentAddress, SproutWitness};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SproutNoteData {
    address: SproutPaymentAddress,
    nullifer: Option<u256>,
    witnesses: Vec<SproutWitness>,
    witness_height: i32,
}

impl SproutNoteData {
    pub fn address(&self) -> &SproutPaymentAddress {
        &self.address
    }

    pub fn nullifer(&self) -> Option<&u256> {
        self.nullifer.as_ref()
    }

    pub fn witnesses(&self) -> &[SproutWitness] {
        &self.witnesses
    }

    pub fn witness_height(&self) -> i32 {
        self.witness_height
    }
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
