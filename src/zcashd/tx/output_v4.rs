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
    cv: u256,
    cmu: u256,
    ephemeral_key: u256,
    enc_ciphertext: Blob<ENC_CIPHERTEXT_SIZE>,
    out_ciphertext: Blob<OUT_CIPHERTEXT_SIZE>,
    zkproof: GrothProof,
}

impl Parse for OutputV4 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let cv = parse!(parser, "cv")?;
        let cmu = parse!(parser, "cmu")?;
        let ephemeral_key = parse!(parser, "ephemeral_key")?;
        let enc_ciphertext = parse!(parser, "enc_ciphertext")?;
        let out_ciphertext = parse!(parser, "out_ciphertext")?;
        let zkproof = parse!(parser, "zkproof")?;
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
