use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SproutPaymentAddress {
    a_pk: u256,
    pk_enc: u256,
}

impl SproutPaymentAddress {
    pub fn a_pk(&self) -> &u256 {
        &self.a_pk
    }

    pub fn pk_enc(&self) -> &u256 {
        &self.pk_enc
    }
}

impl Parse for SproutPaymentAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        let a_pk = parse!(p, "sprout payment address a_pk")?;
        let pk_enc = parse!(p, "sprout payment address pk_enc")?;
        Ok(Self {
            a_pk,
            pk_enc,
        })
    }
}
