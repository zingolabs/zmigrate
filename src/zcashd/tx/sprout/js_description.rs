use anyhow::Result;

use crate::{Amount, ParseWithParam, Parser, parse, u256};

use super::{NoteEncryptionCiphertext, SproutProof};

const ZC_NUM_JS_INPUTS: usize = 2;
const ZC_NUM_JS_OUTPUTS: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct JSDescription {
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

impl ParseWithParam<bool> for JSDescription {
    fn parse(p: &mut Parser, use_groth: bool) -> Result<Self> {
        Ok(Self {
            vpub_old: parse!(p, "vpub_old")?,
            vpub_new: parse!(p, "vpub_new")?,
            anchor: parse!(p, "anchor")?,
            nullifiers: parse!(p, "nullifiers")?,
            commitments: parse!(p, "commitments")?,
            ephemeral_key: parse!(p, "ephemeral_key")?,
            random_seed: parse!(p, "random_seed")?,
            macs: parse!(p, "macs")?,
            zkproof: parse!(p, param = use_groth, "zkproof")?,
            ciphertexts: parse!(p, "ciphertexts")?,
        })
    }
}
