use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{Amount, OutputV4, SpendV4};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SaplingBundle {
    amount: Amount,
    spends: Vec<SpendV4>,
    outputs: Vec<OutputV4>,
}

impl SaplingBundle {
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn have_actions(&self) -> bool {
        !(self.spends.is_empty() && self.outputs.is_empty())
    }
}

impl Parse for SaplingBundle {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        let amount = parse!(p, "amount")?;
        let spends = parse!(p, "spends")?;
        let outputs = parse!(p, "outputs")?;
        Ok(Self {
            amount,
            spends,
            outputs,
        })
    }
}
