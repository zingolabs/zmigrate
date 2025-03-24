use anyhow::Result;

use crate::parse;
use crate::zewif::parser::prelude::*;
use crate::zewif::Bip39Mnemonic;

impl Parse for Bip39Mnemonic {
    fn parse(p: &mut Parser) -> Result<Self> {
        let language = parse!(p, "language")?;
        let mnemonic = parse!(p, "mnemonic")?;
        let mut bip39_mnemonic = Self::new(mnemonic);
        bip39_mnemonic.set_language(language);
        Ok(bip39_mnemonic)
    }
}
