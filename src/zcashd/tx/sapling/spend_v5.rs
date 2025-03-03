use anyhow::Result;

use crate::{parse, u256, Blob64, Parse, Parser};

use super::super::GrothProof;

#[derive(Debug, Clone, PartialEq)]
pub struct SpendV5 {
    pub cv: u256,
    pub nullifier: u256,
    pub rk: u256,
}

impl SpendV5 {
    pub fn into_spend_description(
        self,
        anchor: u256,
        zkproof: GrothProof,
        spend_auth_sig: Blob64,
    ) -> SpendDescription {
        SpendDescription {
            cv: self.cv,
            anchor,
            nullifier: self.nullifier,
            rk: self.rk,
            zkproof,
            spend_auth_sig,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpendDescription {
    cv: u256,
    anchor: u256,
    nullifier: u256,
    rk: u256,
    zkproof: GrothProof,
    spend_auth_sig: Blob64,
}

impl Parse for SpendV5 {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            cv: parse!(p, "cv")?,
            nullifier: parse!(p, "nullifier")?,
            rk: parse!(p, "rk")?,
        })
    }
}
