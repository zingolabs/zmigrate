use anyhow::Result;

use crate::{ parse, u256, Blob, Parse, Parser };

use super::GrothProof;

#[derive(Debug, Clone, PartialEq)]
pub struct SpendV4 {
    cv: u256,
    anchor: u256,
    nullifier: u256,
    rk: u256,
    zkproof: GrothProof,
    spend_auth_sig: Blob<64>,
}

impl Parse for SpendV4 {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
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
