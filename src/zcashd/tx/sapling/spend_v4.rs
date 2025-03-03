use anyhow::Result;

use crate::{parse, u256, Blob, Parse, Parser};

use super::super::GrothProof;

#[derive(Debug, Clone, PartialEq)]
pub struct SpendV4 {
    pub cv: u256,
    pub anchor: u256,
    pub nullifier: u256,
    pub rk: u256,
    pub zkproof: GrothProof,
    pub spend_auth_sig: Blob<64>,
}

impl Parse for SpendV4 {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            cv: parse!(p, "cv")?,
            anchor: parse!(p, "anchor")?,
            nullifier: parse!(p, "nullifier")?,
            rk: parse!(p, "rk")?,
            zkproof: parse!(p, "zkproof")?,
            spend_auth_sig: parse!(p, "spend_auth_sig")?,
        })
    }
}
