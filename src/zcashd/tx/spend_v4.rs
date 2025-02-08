use anyhow::{ Result, Context };

use crate::{ u256, Blob, Parse, Parser };

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
        let cv = Parse::parse(parser).context("Parsing cv")?;
        let anchor = Parse::parse(parser).context("Parsing anchor")?;
        let nullifier = Parse::parse(parser).context("Parsing nullifier")?;
        let rk = Parse::parse(parser).context("Parsing rk")?;
        let zkproof = Parse::parse(parser).context("Parsing zkproof")?;
        let spend_auth_sig = Parse::parse(parser).context("Parsing spend_auth_sig")?;
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
