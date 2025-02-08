use anyhow::{ Result, Context };

use crate::{ u256, Parse, Parser };

use super::{ NoteEncryptionCiphertext, Amount, SproutProof };

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
    pub ciphertexts: [NoteEncryptionCiphertext; ZC_NUM_JS_OUTPUTS],
}

impl JoinSplitDesc {
    pub fn parse(parser: &mut Parser, use_groth: bool) -> Result<Self> {
        let vpub_old = Parse::parse(parser).context("vpub_old")?;
        let vpub_new = Parse::parse(parser).context("vpub_new")?;
        let anchor = Parse::parse(parser).context("anchor")?;
        let nullifiers = Parse::parse(parser).context("nullifiers")?;
        let commitments = Parse::parse(parser).context("commitments")?;
        let ephemeral_key = Parse::parse(parser).context("ephemeral_key")?;
        let random_seed = Parse::parse(parser).context("random_seed")?;
        let macs = Parse::parse(parser).context("macs")?;
        let zkproof = SproutProof::parse(parser, use_groth).context("zkproof")?;
        let ciphertexts = Parse::parse(parser).context("ciphertexts")?;

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
