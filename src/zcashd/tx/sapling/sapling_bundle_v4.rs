use anyhow::Result;

use crate::{Amount, Blob64, Parse, Parser, parse};

use super::super::{OutputV4, SpendV4};

#[derive(Debug, Clone, PartialEq)]
pub struct SaplingBundleV4 {
    amount: Amount,
    spends: Vec<SpendV4>,
    outputs: Vec<OutputV4>,
    binding_sig: Option<Blob64>,
}

impl SaplingBundleV4 {
    pub fn have_actions(&self) -> bool {
        !(self.spends.is_empty() && self.outputs.is_empty())
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn spends(&self) -> &Vec<SpendV4> {
        &self.spends
    }

    pub fn outputs(&self) -> &Vec<OutputV4> {
        &self.outputs
    }

    pub fn binding_sig(&self) -> &Option<Blob64> {
        &self.binding_sig
    }

    pub fn set_binding_sig(&mut self, binding_sig: Blob64) {
        self.binding_sig = Some(binding_sig);
    }
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

impl Parse for SaplingBundleV4 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let amount = parse!(p, "amount")?;
        let spends = parse!(p, "spends")?;
        let outputs = parse!(p, "outputs")?;
        Ok(Self {
            amount,
            spends,
            outputs,
            binding_sig: None,
        })
    }
}
