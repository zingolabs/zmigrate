use anyhow::Result;

use crate::{ parse, Blob64, Parse, Parser, Amount, OutputV4, SpendV4 };

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SaplingBundle {
    pub amount: Amount,
    pub spends: Vec<SpendV4>,
    pub outputs: Vec<OutputV4>,
    pub binding_sig: Option<Blob64>,
}

impl SaplingBundle {
    pub fn have_actions(&self) -> bool {
        !(self.spends.is_empty() && self.outputs.is_empty())
    }
}

impl Parse for SaplingBundle {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            amount: parse!(p, "amount")?,
            spends: parse!(p, "spends")?,
            outputs: parse!(p, "outputs")?,
            binding_sig: None,
        })
    }
}
