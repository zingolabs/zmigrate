use anyhow::Result;

use crate::{parse, parser::prelude::*};
use crate::Bip39Mnemonic;

impl Parse for Bip39Mnemonic {
    fn parse(p: &mut Parser) -> Result<Self> {
        let language = parse!(p, "language")?;
        let mnemonic = parse!(p, "mnemonic")?;
        let mut bip39_mnemonic = Self::new(mnemonic);
        bip39_mnemonic.set_language(language);
        Ok(bip39_mnemonic)
    }
}
