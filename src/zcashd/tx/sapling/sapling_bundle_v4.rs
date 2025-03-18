use anyhow::Result;

use crate::{parse, Amount, Blob64, Parse, Parser};

use super::super::{OutputV4, SpendV4};

#[derive(Debug, Clone, PartialEq)]
pub struct SaplingBundleV4 {
    pub amount: Amount,
    pub spends: Vec<SpendV4>,
    pub outputs: Vec<OutputV4>,
    pub binding_sig: Option<Blob64>,
}

impl Default for SaplingBundleV4 {
    fn default() -> Self {
        Self {
            amount: Amount::zero(),
            spends: Vec::new(),
            outputs: Vec::new(),
            binding_sig: None,
        }
    }
}

impl SaplingBundleV4 {
    pub fn have_actions(&self) -> bool {
        !(self.spends.is_empty() && self.outputs.is_empty())
    }
}

impl Parse for SaplingBundleV4 {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            amount: parse!(p, "amount")?,
            spends: parse!(p, "spends")?,
            outputs: parse!(p, "outputs")?,
            binding_sig: None,
        })
    }
}
