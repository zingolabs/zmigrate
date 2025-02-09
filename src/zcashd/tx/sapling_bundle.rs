use anyhow::Result;

use crate::{parse, Blob64, Parse, Parser};

use super::{Amount, OutputV4, SpendV4};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SaplingBundle {
    amount: Amount,
    spends: Vec<SpendV4>,
    outputs: Vec<OutputV4>,
    binding_sig: Option<Blob64>,
}

impl SaplingBundle {
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn spends(&self) -> &[SpendV4] {
        &self.spends
    }

    pub fn outputs(&self) -> &[OutputV4] {
        &self.outputs
    }

    pub fn binding_sig(&self) -> Option<&Blob64> {
        self.binding_sig.as_ref()
    }

    pub fn set_binding_sig(&mut self, binding_sig: Blob64) {
        self.binding_sig = Some(binding_sig);
    }

    pub fn have_actions(&self) -> bool {
        !(self.spends.is_empty() && self.outputs.is_empty())
    }
}

impl Parse for SaplingBundle {
    fn parse(p: &mut Parser) -> Result<Self> {
        let amount = parse!(p, "amount")?;
        let spends = parse!(p, "spends")?;
        let outputs = parse!(p, "outputs")?;
        let binding_sig = None;
        Ok(Self {
            amount,
            spends,
            outputs,
            binding_sig,
        })
    }
}
