use anyhow::{ Result, Context };

use crate::{ u256, Parseable, Parser };

use super::{ note_encryption::Ciphertext, Amount, SproutProof };

const ZC_NUM_JS_INPUTS: usize = 2;
const ZC_NUM_JS_OUTPUTS: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct JoinSplitDesc {
    pub vpub_old: Amount,
    pub vpub_new: Amount,
    pub anchor: u256,
    pub nullifiers: [u256; ZC_NUM_JS_INPUTS],
    pub commitments: [u256; ZC_NUM_JS_OUTPUTS],
    pub ephemeral_key: u256,
    pub random_seed: u256,
    pub macs: [u256; ZC_NUM_JS_INPUTS],
    pub zkproof: SproutProof,
    pub ciphertexts: [Ciphertext; ZC_NUM_JS_OUTPUTS],
}

impl JoinSplitDesc {
    pub fn parse(parser: &mut Parser, use_groth: bool) -> Result<Self> {
        let vpub_old = Parseable::parse(parser).context("Parsing vpub_old")?;
        let vpub_new = Parseable::parse(parser).context("Parsing vpub_new")?;
        let anchor = Parseable::parse(parser).context("Parsing anchor")?;
        let nullifiers = Parseable::parse(parser).context("Parsing nullifiers")?;
        let commitments = Parseable::parse(parser).context("Parsing commitments")?;
        let ephemeral_key = Parseable::parse(parser).context("Parsing ephemeral_key")?;
        let random_seed = Parseable::parse(parser).context("Parsing random_seed")?;
        let macs = Parseable::parse(parser).context("Parsing macs")?;
        let zkproof = SproutProof::parse(parser, use_groth).context("Parsing zkproof")?;
        let ciphertexts = Parseable::parse(parser).context("Parsing ciphertexts")?;

        Ok(Self {
            vpub_old,
            vpub_new,
            anchor,
            nullifiers,
            commitments,
            ephemeral_key,
            random_seed,
            macs,
            zkproof,
            ciphertexts,
        })
    }
}
