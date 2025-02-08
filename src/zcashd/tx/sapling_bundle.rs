use anyhow::{Result, Context};

use crate::{Parseable, Parser};

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

impl Parseable for SaplingBundle {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let amount = Amount::parse(parser)
            .context("Parsing SaplingBundle amount")?;
        let spends = Vec::parse(parser)
            .context("Parsing SaplingBundle spends")?;
        let outputs = Vec::parse(parser)
            .context("Parsing SaplingBundle outputs")?;
        Ok(Self {
            amount,
            spends,
            outputs,
        })
    }
}
