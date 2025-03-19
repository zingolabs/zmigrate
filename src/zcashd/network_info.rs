use anyhow::{bail, Result};

use zcash_address::Network;

use crate::{parse, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfo {
    zcash: String,
    identifier: String,
    network: Network,
}

impl NetworkInfo {
    pub fn zcash(&self) -> &str {
        &self.zcash
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn network(&self) -> Network {
        self.network
    }
}

impl Parse for NetworkInfo {
    fn parse(p: &mut Parser) -> Result<Self> {
        let (zcash, identifier): (String, String) = parse!(p, "(zcash, identifier)")?;
        let network = network_for_identifier(&identifier)?;
        Ok(Self { zcash, identifier, network })
    }
}

pub fn network_for_identifier(identifier: &str) -> Result<Network> {
    if identifier == "main" {
        Ok(Network::Main)
    } else if identifier == "test" {
        Ok(Network::Test)
    } else if identifier == "regtest" {
        Ok(Network::Regtest)
    } else {
        bail!("Invalid network identifier: {}", identifier)
    }
}
