use anyhow::Result;

use crate::{parse, Blob, Blob32, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrchardRawAddress {
    pub diversifier: Blob<11>,
    pub pk_d: Blob32,
}

impl Parse for OrchardRawAddress {
    fn parse(p: &mut Parser) -> Result<Self> {
        let diversifier = parse!(p, "diversifier")?;
        let pk_d = parse!(p, "pk_d")?;
        Ok(OrchardRawAddress { diversifier, pk_d })
    }
}
