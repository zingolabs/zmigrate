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
        Ok(Self {
            address: parse!(p, "address")?,
            nullifer: parse!(p, "nullifer")?,
            witnesses: parse!(p, "witnesses")?,
            witness_height: parse!(p, "witness_height")?,
        })
    }
}
