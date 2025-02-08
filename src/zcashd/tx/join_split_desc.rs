use anyhow::{ Result, Context };

use crate::{ parse, u256, Parser };

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
    pub fn parse(p: &mut Parser, use_groth: bool) -> Result<Self> {
        let vpub_old = parse!(p, "vpub_old")?;
        let vpub_new = parse!(p, "vpub_new")?;
        let anchor = parse!(p, "anchor")?;
        let nullifiers = parse!(p, "nullifiers")?;
        let commitments = parse!(p, "commitments")?;
        let ephemeral_key = parse!(p, "ephemeral_key")?;
        let random_seed = parse!(p, "random_seed")?;
        let macs = parse!(p, "macs")?;
        let zkproof = SproutProof::parse(p, use_groth).context("zkproof")?;
        let ciphertexts = parse!(p, "ciphertexts")?;

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
