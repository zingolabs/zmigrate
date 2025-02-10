use anyhow::Result;

use crate::{parse, u256, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SproutPaymentAddress {
    pub a_pk: u256,
    pub pk_enc: u256,
}

impl Parse for SproutPaymentAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            a_pk: parse!(p, "sprout payment address a_pk")?,
            pk_enc: parse!(p, "sprout payment address pk_enc")?,
        })
    }
}
