use anyhow::Result;

use crate::{Data, Parse, Parser, parse};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Script(pub Data);

impl Parse for Script {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "Script")?))
    }
}

impl std::fmt::Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Script<{}>({})", self.0.len(), hex::encode(self))
    }
}

impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        &self.0.0
    }
}
