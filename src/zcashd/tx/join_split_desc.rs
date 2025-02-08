use anyhow::{ Result, Context };

use crate::{ parse, u256, Parse, Parser };

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
        let vpub_old = parse!(parser, "vpub_old")?;
        let vpub_new = parse!(parser, "vpub_new")?;
        let anchor = parse!(parser, "anchor")?;
        let nullifiers = parse!(parser, "nullifiers")?;
        let commitments = parse!(parser, "commitments")?;
        let ephemeral_key = parse!(parser, "ephemeral_key")?;
        let random_seed = parse!(parser, "random_seed")?;
        let macs = parse!(parser, "macs")?;
        let zkproof = SproutProof::parse(parser, use_groth).context("zkproof")?;
        let ciphertexts = parse!(parser, "ciphertexts")?;

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
