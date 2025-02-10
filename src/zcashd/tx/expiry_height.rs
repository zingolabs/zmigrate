use anyhow::Result;

use crate::{ parse, Parse, Parser };

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpiryHeight(pub u32);

impl ExpiryHeight {
    pub fn from_u32(expiry_height: u32) -> Self {
        Self(expiry_height)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_option(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(self)
        }
    }
}

impl Parse for ExpiryHeight {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(ExpiryHeight::from_u32(parse!(p, "expiry_height")?))
    }
}
