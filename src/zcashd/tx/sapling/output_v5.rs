use anyhow::Result;

use crate::{parse, u256, Blob, Parse, Parser};

use super::super::GrothProof;

#[derive(Debug, Clone, PartialEq)]
pub struct OutputV5 {
    pub cv: u256,
    pub cmu: u256,
    pub ephemeral_key: u256,
    pub enc_ciphertext: Blob<580>,
    pub out_ciphertext: Blob<80>,
}

impl OutputV5 {
    pub fn into_output_description(self, zkproof: GrothProof) -> OutputDescription {
        OutputDescription {
            cv: self.cv,
            cmu: self.cmu,
            ephemeral_key: self.ephemeral_key,
            enc_ciphertext: self.enc_ciphertext,
            out_ciphertext: self.out_ciphertext,
            zkproof,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputDescription {
    cv: u256,
    cmu: u256,
    ephemeral_key: u256,
    enc_ciphertext: Blob<580>,
    out_ciphertext: Blob<80>,
    zkproof: GrothProof,
}

impl Parse for OutputV5 {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            cv: parse!(p, "cv")?,
            cmu: parse!(p, "cmu")?,
            ephemeral_key: parse!(p, "ephemeral_key")?,
            enc_ciphertext: parse!(p, "enc_ciphertext")?,
            out_ciphertext: parse!(p, "out_ciphertext")?,
        })
    }
}
