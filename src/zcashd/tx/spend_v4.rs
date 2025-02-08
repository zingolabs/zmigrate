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
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let cv = parse!(parser, "cv")?;
        let anchor = parse!(parser, "anchor")?;
        let nullifier = parse!(parser, "nullifier")?;
        let rk = parse!(parser, "rk")?;
        let zkproof = parse!(parser, "zkproof")?;
        let spend_auth_sig = parse!(parser, "spend_auth_sig")?;
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
