use anyhow::Result;

use crate::{parse, Bip39Mnemonic, Parse, Parser};

impl Parse for Bip39Mnemonic {
    fn parse(p: &mut Parser) -> Result<Self> {
        let language = parse!(p, "language")?;
        let mnemonic = parse!(p, "mnemonic")?;
        let fingerprint = None;
        Ok(Self::new(mnemonic, language, fingerprint))
    }
}
