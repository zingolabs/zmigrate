use anyhow::Result;

use crate::{ parse, u256, Blob, Parse, Parser };

use super::GrothProof;

const COMPACT_NOTE_SIZE: usize = 1 + // version
    11 + // diversifier
    8  + // value
    32; // rseed (or rcm prior to ZIP 212)
const AEAD_TAG_SIZE: usize = 16;
const NOTE_PLAINTEXT_SIZE: usize = COMPACT_NOTE_SIZE + 512;
const ENC_CIPHERTEXT_SIZE: usize = NOTE_PLAINTEXT_SIZE + AEAD_TAG_SIZE;

const OUT_PLAINTEXT_SIZE: usize = 32 + // pk_d
    32; // esk
const OUT_CIPHERTEXT_SIZE: usize = OUT_PLAINTEXT_SIZE + AEAD_TAG_SIZE;

#[derive(Debug, Clone, PartialEq)]
pub struct OutputV4 {
    pub cv: u256,
    pub cmu: u256,
    pub ephemeral_key: u256,
    pub enc_ciphertext: Blob<ENC_CIPHERTEXT_SIZE>,
    pub out_ciphertext: Blob<OUT_CIPHERTEXT_SIZE>,
    pub zkproof: GrothProof,
}

impl Parse for OutputV4 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let cv = parse!(p, "cv")?;
        let cmu = parse!(p, "cmu")?;
        let ephemeral_key = parse!(p, "ephemeral_key")?;
        let enc_ciphertext = parse!(p, "enc_ciphertext")?;
        let out_ciphertext = parse!(p, "out_ciphertext")?;
        let zkproof = parse!(p, "zkproof")?;
        Ok(Self {
            cv,
            cmu,
            ephemeral_key,
            enc_ciphertext,
            out_ciphertext,
            zkproof,
        })
    }
}
