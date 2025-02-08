use anyhow::{ Result, Context };

use crate::{ u256, Blob, Parse, Parser };

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
        let cv = u256::parse(parser).context("Parsing cv")?;
        let cmu = u256::parse(parser).context("Parsing cmu")?;
        let ephemeral_key = u256::parse(parser).context("Parsing ephemeral_key")?;
        let enc_ciphertext = Blob::parse(parser).context("Parsing enc_ciphertext")?;
        let out_ciphertext = Blob::parse(parser).context("Parsing out_ciphertext")?;
        let zkproof = GrothProof::parse(parser).context("Parsing zkproof")?;
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
