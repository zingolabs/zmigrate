use anyhow::{bail, Result};

use crate::{parse, Parser};

pub trait Versioned {
    const VERSION: u8;

    fn get_version(p: &mut Parser) -> Result<u8> {
        let version = parse!(p, u8, "version")?;
        if version > Self::VERSION {
            bail!("Wallet file version \"{}\" is from future version of zingo", version);
        }
        Ok(version)
    }
}
