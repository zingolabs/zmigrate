use anyhow::Result;

use crate::{parse, Blob, Blob32, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingZPaymentAddress {
    pub diversifier: Blob<11>,
    pub pk: Blob32,
}

impl Parse for SaplingZPaymentAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        let diversifier = parse!(p, "diversifier")?;
        let pk = parse!(p, "pk")?;
        Ok(SaplingZPaymentAddress { diversifier, pk })
    }
}
