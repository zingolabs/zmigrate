use anyhow::Result;

use crate::{ parse, u256, Blob, Parse, Parser };

use super::GrothProof;

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
        let cv = parse!(p, "cv")?;
        let anchor = parse!(p, "anchor")?;
        let nullifier = parse!(p, "nullifier")?;
        let rk = parse!(p, "rk")?;
        let zkproof = parse!(p, "zkproof")?;
        let spend_auth_sig = parse!(p, "spend_auth_sig")?;
        Ok(Self {
            cv,
            anchor,
            nullifier,
            rk,
            zkproof,
            spend_auth_sig,
        })
    }
}
