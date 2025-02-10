use anyhow::Result;

use crate::{parse, Data, Parse, Parser};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Script(pub Data);

impl Script {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Parse for Script {
    fn parse(p: &mut Parser) -> Result<Self> {
        let data = parse!(p, "script data")?;
        Ok(Self(data))
    }
}

impl std::fmt::Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Script<{}>({})", self.len(), hex::encode(self))
    }
}

impl AsRef<Data> for Script {
    fn as_ref(&self) -> &Data {
        &self.0
    }
}

impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        &self.0.0
    }
}
