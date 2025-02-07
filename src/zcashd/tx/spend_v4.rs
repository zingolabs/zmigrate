use anyhow::{ Result, Context };

use crate::{ u256, Blob, Parseable };

use super::GROTH_PROOF_SIZE;

#[derive(Debug, Clone, PartialEq)]
pub struct SpendV4 {
    cv: u256,
    anchor: u256,
    nullifier: u256,
    rk: u256,
    zkproof: Blob<GROTH_PROOF_SIZE>,
    spend_auth_sig: Blob<64>,
}

impl Parseable for SpendV4 {
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let cv = u256::parse(parser).context("Parsing cv")?;
        let anchor = u256::parse(parser).context("Parsing anchor")?;
        let nullifier = u256::parse(parser).context("Parsing nullifier")?;
        let rk = u256::parse(parser).context("Parsing rk")?;
        let zkproof = Blob::parse(parser).context("Parsing zkproof")?;
        let spend_auth_sig = Blob::parse(parser).context("Parsing spend_auth_sig")?;
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
