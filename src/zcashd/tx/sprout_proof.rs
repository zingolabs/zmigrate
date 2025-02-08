use anyhow::{ Result, Context };

use crate::{ Parse, Parser };

use super::{GrothProof, PHGRProof};

#[derive(Debug, Clone, PartialEq)]
pub enum SproutProof {
    PHGRProof(PHGRProof),
    GrothProof(GrothProof),
}

impl SproutProof {
    pub fn parse(parser: &mut Parser, use_groth: bool) -> Result<Self> where Self: Sized {
        if use_groth {
            let groth_proof = Parse::parse(parser).context("groth proof")?;
            Ok(Self::GrothProof(groth_proof))
        } else {
            let phgr_proof = Parse::parse(parser).context("phgr proof")?;
            Ok(Self::PHGRProof(phgr_proof))
        }
    }
}
